//! HTTP transport: routing, middleware, and per-domain handlers. Depends
//! on the domain modules but the domain modules do not depend on it (no
//! HTTP types leak into `lesson`, `runner`, or `tutor`).

mod chat;
mod lessons;
mod ratelimit;
mod run;
mod static_assets;

use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::extract::DefaultBodyLimit;
use axum::extract::Request;
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::routing::{get, post};
use axum::Router;
use tracing::info;

use crate::lesson::Repository;
use crate::runner::Runner;
use crate::tutor::Service as Tutor;

use self::ratelimit::ChatLimiter;

/// The dependency graph passed into every handler via [`axum::extract::State`].
#[derive(Clone)]
pub struct AppState {
    pub lessons: Arc<dyn Repository>,
    pub runner: Arc<dyn Runner>,
    pub tutor: Arc<Tutor>,
    pub chat_limiter: Arc<ChatLimiter>,
}

/// Builds the fully-wired axum router.
pub fn router(lessons: Arc<dyn Repository>, runner: Arc<dyn Runner>, tutor: Arc<Tutor>) -> Router {
    let chat_limiter = Arc::new(ChatLimiter::from_env());
    let max_body = chat_limiter.limits.max_body_bytes as usize;
    let state = AppState {
        lessons,
        runner,
        tutor,
        chat_limiter,
    };

    Router::new()
        .route("/healthz", get(health))
        .route("/api/lessons", get(lessons::list))
        .route("/api/lessons/:id", get(lessons::by_id))
        .route("/api/run", post(run::handle))
        .route(
            "/api/chat",
            post(chat::handle).layer(DefaultBodyLimit::max(max_body)),
        )
        .route("/api/chat/status", get(chat::status))
        .fallback(static_assets::serve)
        .layer(middleware::from_fn(log_requests))
        .with_state(state)
}

/// A cheap, dependency-free probe used by orchestrators (Cloud Run,
/// Kubernetes, etc.) to confirm the process is up.
async fn health() -> ([(&'static str, &'static str); 1], &'static str) {
    ([("Cache-Control", "no-store")], "ok\n")
}

/// Tiny middleware that logs each request. `/healthz` is skipped because
/// Cloud Run / k8s hammer it on a short interval and flooding the logs
/// with probe pings is pure noise.
async fn log_requests(req: Request, next: Next) -> Response {
    let path = req.uri().path().to_string();
    if path == "/healthz" {
        return next.run(req).await;
    }
    let method = req.method().clone();
    let start = Instant::now();
    let res = next.run(req).await;
    let elapsed: Duration = start.elapsed();
    info!("{method} {path} {elapsed:?}");
    res
}
