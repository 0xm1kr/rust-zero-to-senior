use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::AppState;

/// The wire shape posted to /api/run. Kept as a struct so adding
/// (filename, args, env) later is a non-breaking change.
#[derive(Deserialize)]
pub struct RunRequest {
    pub code: String,
}

/// What the UI's playground panel renders. `stdout`/`stderr` are always
/// present (possibly empty); `error` is set only on compile failure, non-
/// zero exit, or timeout. `duration` is a human-readable string ("123ms")
/// so the frontend can display it verbatim.
#[derive(Serialize)]
pub struct RunResponse {
    pub stdout: String,
    pub stderr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub duration: String,
}

/// Caps the size of a single submission to keep the runner (and the
/// Playground API) from being flooded with multi-megabyte payloads.
const MAX_RUN_CODE_BYTES: usize = 64 * 1024;

/// Executes user-submitted Rust source through the configured Runner
/// (Local or Playground) and returns the captured stdout/stderr plus a
/// wall-clock duration. Errors are always returned with HTTP 200 so the
/// UI can render them in the same output panel.
pub async fn handle(
    State(s): State<AppState>,
    body: Result<Json<RunRequest>, axum::extract::rejection::JsonRejection>,
) -> impl IntoResponse {
    let req = match body {
        Ok(Json(b)) => b,
        Err(_) => return (StatusCode::BAD_REQUEST, "bad json").into_response(),
    };
    if req.code.len() > MAX_RUN_CODE_BYTES {
        return (StatusCode::PAYLOAD_TOO_LARGE, "code too large").into_response();
    }

    let start = Instant::now();
    let result = s.runner.run(&req.code).await;
    let elapsed = start.elapsed();

    let resp = RunResponse {
        stdout: result.stdout,
        stderr: result.stderr,
        error: result.error,
        duration: format_duration(elapsed),
    };
    Json(resp).into_response()
}

/// "123ms" / "1.2s" — round to milliseconds for readability.
fn format_duration(d: std::time::Duration) -> String {
    let ms = d.as_millis();
    if ms < 1000 {
        format!("{ms}ms")
    } else {
        format!("{:.2}s", d.as_secs_f64())
    }
}
