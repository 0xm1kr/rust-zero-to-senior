//! Binary entrypoint for the Rust tutorial app.
//!
//! This file does ONE job: composition root. It loads configuration,
//! constructs each domain object, and starts the HTTP server. All real
//! logic lives in the [`config`], [`lesson`], [`runner`], [`tutor`], and
//! [`api`] modules.

mod api;
mod config;
mod lesson;
mod runner;
mod tutor;

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::signal;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    // tracing: respect RUST_LOG, default to info.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .compact()
        .init();

    // 1. Configuration: optional .env file, real env vars always win.
    match config::load_dotenv(".env") {
        Ok(0) => {}
        Ok(n) => info!("loaded {n} entries from .env"),
        Err(e) => warn!("reading .env: {e}"),
    }

    // 2. Resolve listen address. CLI flag > $PORT (Cloud Run) > :8080.
    let addr = resolve_addr(parse_addr_flag())?;

    // 3. Domain objects.
    let lessons = Arc::new(lesson::InMemoryRepository::new(lesson::catalog()));
    let code_runner = runner::from_env(Duration::from_secs(10));
    let provider = tutor::select_from_env();
    let chat = Arc::new(tutor::Service::new(provider, lessons.clone()));

    info!("code runner: {}", code_runner.backend());
    let status = chat.status();
    if status.available {
        info!("AI chat enabled via {} ({})", status.provider, status.model);
    } else {
        info!(
            "AI chat disabled: {}",
            status.hint.as_deref().unwrap_or("(no provider)")
        );
    }

    // 4. Transport (HTTP).
    let app = api::router(lessons, code_runner, chat);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Rust tutorial listening on http://{addr}");

    // 5. Serve with graceful shutdown. Cloud Run / Kubernetes send SIGTERM
    //    ~10s before killing the container; we drain in-flight requests
    //    before exiting.
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("bye");
    Ok(())
}

/// Tiny argv parser: `-addr :9000` or `--addr=:9000`. We avoid pulling in a
/// CLI crate; the tutorial deliberately keeps deps minimal.
fn parse_addr_flag() -> Option<String> {
    let mut args = env::args().skip(1);
    while let Some(a) = args.next() {
        if let Some(v) = a.strip_prefix("--addr=") {
            return Some(v.to_string());
        }
        if let Some(v) = a.strip_prefix("-addr=") {
            return Some(v.to_string());
        }
        if a == "-addr" || a == "--addr" {
            return args.next();
        }
    }
    None
}

/// resolve_addr picks the listen address with this precedence:
///
///  1. CLI flag (`-addr`) if provided
///  2. `$PORT` environment variable (Cloud Run, App Engine, Heroku, etc.)
///  3. `:8080` fallback
fn resolve_addr(flag: Option<String>) -> anyhow::Result<SocketAddr> {
    let raw = flag
        .filter(|s| !s.is_empty())
        .or_else(|| env::var("PORT").ok().map(|p| format!(":{p}")))
        .unwrap_or_else(|| ":8080".to_string());

    let host_port = if let Some(port) = raw.strip_prefix(':') {
        format!("0.0.0.0:{port}")
    } else {
        raw
    };
    Ok(host_port.parse()?)
}

/// shutdown_signal completes when SIGINT or SIGTERM is received, which axum
/// then turns into a graceful shutdown of the HTTP server.
async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = signal::ctrl_c().await;
    };

    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut s) = signal::unix::signal(signal::unix::SignalKind::terminate()) {
            s.recv().await;
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }
    info!("shutdown signal received, draining...");
}
