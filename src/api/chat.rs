use std::time::Duration;

use axum::extract::ConnectInfo;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tokio::time::timeout;

use crate::tutor::Message;

use super::ratelimit::{client_ip, trust_forwarded_headers, RateLimitError};
use super::AppState;

/// The wire shape POSTed by the frontend. `lesson_id` and `code` give the
/// tutor service the context it needs to build a meaningful system prompt;
/// `messages` is the full conversation history, with the latest user turn
/// at the end.
#[derive(Deserialize)]
pub struct ChatRequest {
    #[serde(default, rename = "lessonId")]
    pub lesson_id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub messages: Vec<Message>,
}

/// Reply envelope. Exactly one of `message` or `error` is populated on a
/// successful HTTP exchange; the frontend renders `error` as a system-style
/// message in the chat UI.
#[derive(Serialize)]
pub struct ChatResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "retryAfter")]
    pub retry_after: Option<u32>,
}

/// Caps how many turns we forward to the LLM per request. The frontend
/// persists chat history in localStorage and may send a very long
/// conversation; trimming here protects against runaway token usage and
/// keeps prompt latency predictable.
const MAX_HISTORY_TURNS: usize = 40;

/// Returns the tutor service's availability snapshot so the frontend can
/// show "AI chat enabled (gemini-2.5-flash)" or a configuration hint
/// without trying a full request first.
pub async fn status(State(s): State<AppState>) -> impl IntoResponse {
    let mut out = serde_json::to_value(s.tutor.status()).expect("status json");
    if let Some(obj) = out.as_object_mut() {
        obj.insert(
            "limits".to_string(),
            serde_json::to_value(&s.chat_limiter.limits).expect("limits json"),
        );
    }
    Json(out)
}

/// Main chat endpoint. Validates the incoming request, trims the history,
/// and asks the tutor service for the next assistant message.
/// Provider/transport errors are returned with HTTP 200 + an `error` field
/// so the UI can render them inline rather than as a hard network failure.
pub async fn handle(
    State(s): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    body: Result<Json<ChatRequest>, axum::extract::rejection::JsonRejection>,
) -> Response {
    let remote = addr.ip().to_string();
    let client = client_ip(&headers, &remote, trust_forwarded_headers());

    let mut req = match body {
        Ok(Json(b)) => b,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ChatResponse {
                    message: None,
                    error: Some("bad json".to_string()),
                    retry_after: None,
                }),
            )
                .into_response();
        }
    };
    if req.messages.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ChatResponse {
                message: None,
                error: Some("no messages".to_string()),
                retry_after: None,
            }),
        )
            .into_response();
    }

    let limits = &s.chat_limiter.limits;
    for msg in &req.messages {
        if msg.role != "user" && msg.role != "assistant" {
            return bad_request("invalid message role");
        }
        if msg.content.len() > limits.max_message_chars as usize {
            return bad_request(format!(
                "message too long (max {} characters)",
                limits.max_message_chars
            ));
        }
    }
    if req.code.len() > limits.max_code_chars as usize {
        return bad_request(format!(
            "code too long (max {} characters)",
            limits.max_code_chars
        ));
    }

    req.messages.retain(|m| !m.content.trim().is_empty());
    if req.messages.is_empty() {
        return bad_request("no messages");
    }
    if req.messages.last().map(|m| m.role.as_str()) != Some("user") {
        return bad_request("last message must be from the user");
    }

    if req.messages.len() > MAX_HISTORY_TURNS {
        let drop_count = req.messages.len() - MAX_HISTORY_TURNS;
        req.messages.drain(..drop_count);
    }

    if let Some(err) = s.chat_limiter.check(&client) {
        return rate_limit_response(err);
    }

    let result = timeout(
        Duration::from_secs(60),
        s.tutor.reply(&req.lesson_id, &req.code, &req.messages),
    )
    .await;

    let resp = match result {
        Ok(Ok(msg)) => ChatResponse {
            message: Some(msg),
            error: None,
            retry_after: None,
        },
        Ok(Err(e)) => ChatResponse {
            message: None,
            error: Some(e.to_string()),
            retry_after: None,
        },
        Err(_) => ChatResponse {
            message: None,
            error: Some("request timed out".to_string()),
            retry_after: None,
        },
    };
    Json(resp).into_response()
}

fn bad_request(msg: impl Into<String>) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(ChatResponse {
            message: None,
            error: Some(msg.into()),
            retry_after: None,
        }),
    )
        .into_response()
}

fn rate_limit_response(err: RateLimitError) -> Response {
    let mut resp = (
        StatusCode::TOO_MANY_REQUESTS,
        Json(ChatResponse {
            message: None,
            error: Some(err.message),
            retry_after: Some(err.retry_after),
        }),
    )
        .into_response();
    resp.headers_mut().insert(
        axum::http::header::RETRY_AFTER,
        err.retry_after.to_string().parse().expect("retry-after"),
    );
    resp
}
