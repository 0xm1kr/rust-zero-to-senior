//! Smoke-tests the curriculum by POSTing every lesson's starter code to
//! `/api/run` and reporting any that fail.
//!
//! Usage:
//!     cargo run --bin smoke
//!     cargo run --bin smoke -- --base http://localhost:8774
//!     cargo run --bin smoke -- --only intro,vec
//!
//! Exit status is 0 on success, 1 on lesson failures, 2 on infrastructure
//! errors (server unreachable, etc.).

use std::collections::HashSet;
use std::process::ExitCode;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Summary {
    id: String,
}

#[derive(Deserialize)]
struct Lesson {
    id: String,
    #[serde(default)]
    code: String,
}

#[derive(Serialize)]
struct RunReq<'a> {
    code: &'a str,
}

#[derive(Deserialize, Default)]
#[allow(dead_code)]
struct RunResp {
    #[serde(default)]
    stdout: String,
    #[serde(default)]
    stderr: String,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    duration: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let base = arg_value(&args, "--base").unwrap_or_else(|| "http://localhost:8080".to_string());
    let only = arg_value(&args, "--only").unwrap_or_default();

    let only: HashSet<String> = only
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Skip lessons that intentionally never terminate.
    let skip: HashSet<&str> = [
        "http-server", // would bind a port
        "http-client", // would hit the network
        "middleware",
        "graceful-shutdown",
    ]
    .into_iter()
    .collect();

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("build client: {e}");
            return ExitCode::from(2);
        }
    };

    let summaries: Vec<Summary> = match client.get(format!("{base}/api/lessons")).send().await {
        Ok(r) => match r.json().await {
            Ok(v) => v,
            Err(e) => {
                eprintln!("decode lessons: {e}");
                return ExitCode::from(2);
            }
        },
        Err(e) => {
            eprintln!("fetch lessons: {e}");
            return ExitCode::from(2);
        }
    };

    let mut failures = Vec::new();
    let mut checked = 0usize;
    for s in summaries {
        if !only.is_empty() && !only.contains(&s.id) {
            continue;
        }
        if skip.contains(s.id.as_str()) {
            println!("[SKIP]  {}", s.id);
            continue;
        }
        let url = format!("{}/api/lessons/{}", base, s.id);
        let lesson: Lesson = match client.get(&url).send().await {
            Ok(r) => match r.json().await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("decode lesson {}: {e}", s.id);
                    return ExitCode::from(2);
                }
            },
            Err(e) => {
                eprintln!("fetch lesson {}: {e}", s.id);
                return ExitCode::from(2);
            }
        };
        if lesson.code.trim().is_empty() {
            println!("[EMPTY] {}", s.id);
            continue;
        }
        checked += 1;

        let body = RunReq { code: &lesson.code };
        let resp = match client
            .post(format!("{base}/api/run"))
            .json(&body)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                println!("[FAIL]  {}  post: {e}", lesson.id);
                failures.push(lesson.id.clone());
                continue;
            }
        };
        let r: RunResp = match resp.json().await {
            Ok(v) => v,
            Err(e) => {
                println!("[FAIL]  {}  decode: {e}", lesson.id);
                failures.push(lesson.id.clone());
                continue;
            }
        };
        if let Some(err) = r.error.as_deref().filter(|s| !s.is_empty()) {
            let snippet = truncate(&r.stderr.replace('\n', " "), 120);
            println!("[FAIL]  {}  {} stderr={snippet:?}", lesson.id, err);
            failures.push(lesson.id);
        } else {
            println!("[OK]    {}  ({})", lesson.id, r.duration);
        }
    }

    println!("\nchecked {checked}, failures {}", failures.len());
    if failures.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    let prefix = format!("{name}=");
    for (i, a) in args.iter().enumerate() {
        if let Some(v) = a.strip_prefix(&prefix) {
            return Some(v.to_string());
        }
        if a == name {
            return args.get(i + 1).cloned();
        }
    }
    None
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() > n {
        let mut out: String = s.chars().take(n).collect();
        out.push('…');
        out
    } else {
        s.to_string()
    }
}
