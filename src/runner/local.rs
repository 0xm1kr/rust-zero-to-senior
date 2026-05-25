use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;

use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::timeout;

use super::{RunResult, Runner};

/// Executes Rust source via `rustc` + execution in a fresh temp directory
/// on the host. The temp dir is deleted before `run` returns.
///
/// SECURITY: user code runs with the same UID and filesystem access as the
/// server process. Suitable for `cargo run` on your laptop; NOT suitable
/// for anything reachable from the public internet — use Playground there.
pub struct Local {
    pub timeout: Duration,
    /// Path to the `rustc` binary; defaults to `"rustc"` on `$PATH`.
    pub rustc: String,
}

impl Local {
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            rustc: "rustc".to_string(),
        }
    }
}

#[async_trait]
impl Runner for Local {
    fn backend(&self) -> &str {
        "local (rustc)"
    }

    async fn run(&self, code: &str) -> RunResult {
        // 1. Temp dir.
        let dir = match tempdir().await {
            Ok(d) => d,
            Err(e) => {
                return RunResult {
                    error: Some(e.to_string()),
                    ..Default::default()
                }
            }
        };
        // Guard cleans up on drop, even on early return paths.
        let _guard = TempDirGuard { path: dir.clone() };

        // 2. Write source.
        let src = dir.join("main.rs");
        if let Err(e) = write_file(&src, code).await {
            return RunResult {
                error: Some(format!("write source: {e}")),
                ..Default::default()
            };
        }

        let bin = dir.join(if cfg!(windows) { "main.exe" } else { "main" });

        // 3. Compile.
        let compile = Command::new(&self.rustc)
            .arg("--edition=2021")
            .arg("-O")
            .arg("-o")
            .arg(&bin)
            .arg(&src)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        let compile = match timeout(self.timeout, compile).await {
            Ok(Ok(out)) => out,
            Ok(Err(e)) => {
                return RunResult {
                    error: Some(format!("rustc launch: {e}")),
                    ..Default::default()
                };
            }
            Err(_) => {
                return RunResult {
                    error: Some(format!("compilation timed out after {:?}", self.timeout)),
                    ..Default::default()
                };
            }
        };

        if !compile.status.success() {
            let stderr = String::from_utf8_lossy(&compile.stderr).into_owned();
            let stdout = String::from_utf8_lossy(&compile.stdout).into_owned();
            return RunResult {
                stderr: if stderr.is_empty() {
                    stdout.clone()
                } else {
                    stderr
                },
                stdout: String::new(),
                error: Some("compile failed".to_string()),
            };
        }

        // 4. Run.
        let exec = Command::new(&bin)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        let exec = match timeout(self.timeout, exec).await {
            Ok(Ok(out)) => out,
            Ok(Err(e)) => {
                return RunResult {
                    error: Some(format!("exec: {e}")),
                    ..Default::default()
                };
            }
            Err(_) => {
                return RunResult {
                    error: Some(format!("execution timed out after {:?}", self.timeout)),
                    ..Default::default()
                };
            }
        };

        let mut result = RunResult {
            stdout: String::from_utf8_lossy(&exec.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&exec.stderr).into_owned(),
            error: None,
        };
        if !exec.status.success() {
            result.error = Some(match exec.status.code() {
                Some(c) => format!("process exited with status {c}"),
                None => "process terminated by signal".to_string(),
            });
        }
        result
    }
}

/// Best-effort `tempfile`-free temp dir. We avoid pulling in the `tempfile`
/// crate because it isn't worth a dependency for ten lines of code.
async fn tempdir() -> std::io::Result<PathBuf> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let base = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = base.join(format!("rust-tut-{nanos:x}"));
    tokio::fs::create_dir_all(&dir).await?;
    Ok(dir)
}

async fn write_file(path: &PathBuf, contents: &str) -> std::io::Result<()> {
    let mut f = tokio::fs::File::create(path).await?;
    f.write_all(contents.as_bytes()).await?;
    f.flush().await?;
    Ok(())
}

/// Drop-guard so we clean the temp dir even on early returns. We do the
/// cleanup synchronously because Drop can't be async.
struct TempDirGuard {
    path: PathBuf,
}
impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
