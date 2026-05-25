//! Executes Rust source code submitted from the browser and returns its
//! stdout / stderr / error. Two backends are provided:
//!
//!  - [`Local`] shells out to `rustc` in a temp dir on the host.
//!  - [`Playground`] POSTs to <https://play.rust-lang.org/execute>.
//!
//! The Playground backend is the only safe choice for any deployment that
//! is reachable from the public internet, because the local backend runs
//! untrusted code with the same privileges as the server process.

mod local;
mod playground;

use async_trait::async_trait;
use std::env;
use std::sync::Arc;
use std::time::Duration;

pub use local::Local;
pub use playground::Playground;

/// What the UI needs to render a run.
#[derive(Debug, Default, Clone)]
pub struct RunResult {
    pub stdout: String,
    pub stderr: String,
    /// Populated on compile failure, non-zero exit, or timeout.
    pub error: Option<String>,
}

/// The contract the API layer depends on. Both [`Local`] and [`Playground`]
/// implement it.
#[async_trait]
pub trait Runner: Send + Sync {
    async fn run(&self, code: &str) -> RunResult;
    /// A short human-readable label, surfaced in startup logs so operators
    /// can confirm which executor is wired up.
    fn backend(&self) -> &str;
}

/// Constructs the configured Runner based on `$RUNNER`:
///
/// - `RUNNER=playground` → Rust Playground API (safe for cloud deploys)
/// - `RUNNER=local`      → shell out to `rustc` locally
/// - unset               → auto: Playground if running on Cloud Run /
///   Knative (detected via `$K_SERVICE`), else Local
pub fn from_env(timeout: Duration) -> Arc<dyn Runner> {
    let raw = env::var("RUNNER").unwrap_or_default();
    match raw.trim().to_ascii_lowercase().as_str() {
        "playground" | "play" => Arc::new(Playground::new(timeout)),
        "local" => Arc::new(Local::new(timeout)),
        "" => {
            // K_SERVICE is set on Cloud Run and other Knative runtimes.
            // Defaulting to Playground there is the safe choice — Local
            // would happily run untrusted code in the container.
            if env::var_os("K_SERVICE").is_some() {
                Arc::new(Playground::new(timeout))
            } else {
                Arc::new(Local::new(timeout))
            }
        }
        other => {
            tracing::warn!("runner: unknown RUNNER={other:?}, falling back to local");
            Arc::new(Local::new(timeout))
        }
    }
}
