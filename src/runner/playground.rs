use std::env;
use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::{RunResult, Runner};

/// The public Rust Playground execute endpoint. Override at runtime with
/// `$PLAYGROUND_URL` (e.g. to point at a self-hosted instance).
pub const DEFAULT_PLAYGROUND_URL: &str = "https://play.rust-lang.org/execute";

/// Executes Rust source via the public Rust Playground API.
///
/// Compared to [`super::Local`], user code:
///   - never touches the server's filesystem
///   - cannot reach the network
///   - runs inside the Playground's hardened sandbox
///   - is capped at a wall-clock limit by the Playground itself
///
/// This is the only safe backend for a public-facing deployment.
pub struct Playground {
    pub url: String,
    pub timeout: Duration,
    client: reqwest::Client,
    backend_label: String,
}

impl Playground {
    pub fn new(timeout: Duration) -> Self {
        let url = env::var("PLAYGROUND_URL")
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| DEFAULT_PLAYGROUND_URL.to_string());

        let client = reqwest::Client::builder()
            .timeout(timeout + Duration::from_secs(5))
            .user_agent("rust-tut/1.0 (+https://github.com/0xm1kr)")
            .build()
            .expect("build reqwest client");

        let backend_label = format!("playground ({url})");
        Self {
            url,
            timeout,
            client,
            backend_label,
        }
    }
}

#[derive(Serialize)]
struct PlaygroundReq<'a> {
    channel: &'a str,
    mode: &'a str,
    edition: &'a str,
    #[serde(rename = "crateType")]
    crate_type: &'a str,
    tests: bool,
    code: &'a str,
    backtrace: bool,
}

#[derive(Deserialize)]
struct PlaygroundResp {
    #[serde(default)]
    success: bool,
    #[serde(default)]
    stdout: String,
    #[serde(default)]
    stderr: String,
    #[serde(default)]
    error: Option<String>,
}

#[async_trait]
impl Runner for Playground {
    fn backend(&self) -> &str {
        &self.backend_label
    }

    async fn run(&self, code: &str) -> RunResult {
        let body = PlaygroundReq {
            channel: "stable",
            mode: "debug",
            edition: "2021",
            crate_type: "bin",
            tests: false,
            code,
            backtrace: false,
        };

        let req = self
            .client
            .post(&self.url)
            .json(&body)
            .header("Accept", "application/json")
            .send();

        let res = match tokio::time::timeout(self.timeout + Duration::from_secs(5), req).await {
            Ok(Ok(r)) => r,
            Ok(Err(e)) => {
                return RunResult {
                    error: Some(format!("playground: call: {e}")),
                    ..Default::default()
                };
            }
            Err(_) => {
                return RunResult {
                    error: Some("playground: request timed out".to_string()),
                    ..Default::default()
                };
            }
        };

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            let trimmed: String = body.chars().take(2048).collect();
            return RunResult {
                error: Some(format!(
                    "playground: HTTP {}: {}",
                    status.as_u16(),
                    trimmed.trim()
                )),
                ..Default::default()
            };
        }

        let parsed: PlaygroundResp = match res.json().await {
            Ok(p) => p,
            Err(e) => {
                return RunResult {
                    error: Some(format!("playground: decode: {e}")),
                    ..Default::default()
                };
            }
        };

        let mut out = RunResult {
            stdout: parsed.stdout,
            stderr: parsed.stderr,
            error: None,
        };

        // The Playground returns `success: false` for compile errors and
        // non-zero exits. `error` is set on infrastructure failures.
        if let Some(msg) = parsed
            .error
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            out.error = Some(msg.to_string());
        } else if !parsed.success {
            // Surface a short error string; the actual diagnostic is in
            // stderr, which the UI already renders. We skip the Cargo
            // "Compiling …" / "Running …" preamble and look for the first
            // `error[E…]:` or `error:` line, falling back to a generic
            // label.
            let summary = extract_error_summary(&out.stderr).unwrap_or("compile failed");
            out.error = Some(summary.to_string());
        }
        out
    }
}

/// Picks the first useful diagnostic line out of cargo/rustc stderr. Skips
/// the "Compiling …" / "Finished …" / "Running …" cargo noise so the UI
/// summary shows the actual problem.
fn extract_error_summary(stderr: &str) -> Option<&str> {
    let candidate = stderr.lines().find(|line| {
        let t = line.trim_start();
        t.starts_with("error[") || t.starts_with("error:")
    });
    candidate.or_else(|| {
        stderr.lines().find(|l| {
            let t = l.trim();
            !t.is_empty()
                && !t.starts_with("Compiling")
                && !t.starts_with("Finished")
                && !t.starts_with("Running")
        })
    })
}
