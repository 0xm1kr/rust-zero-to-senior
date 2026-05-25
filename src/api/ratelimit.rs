//! In-memory rate limiting for the chat endpoint.

use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::HeaderMap;
use serde::Serialize;

fn env_int(name: &str, default: u32) -> u32 {
    env::var(name)
        .ok()
        .and_then(|v| v.trim().parse::<u32>().ok())
        .unwrap_or(default)
}

/// Trust `X-Forwarded-For` when behind a known reverse proxy.
pub fn trust_forwarded_headers() -> bool {
    match env::var("TRUST_PROXY")
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .as_deref()
    {
        Some("1" | "true" | "yes") => true,
        _ => env::var("K_SERVICE").map(|v| !v.trim().is_empty()).unwrap_or(false),
    }
}

/// Derive a stable client key from the request.
pub fn client_ip(headers: &HeaderMap, remote_addr: &str, trust_forwarded: bool) -> String {
    if trust_forwarded {
        if let Some(v) = headers
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
        {
            if let Some(first) = v.split(',').next() {
                let ip = first.trim();
                if !ip.is_empty() {
                    return ip.to_string();
                }
            }
        }
        if let Some(v) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
            let ip = v.trim();
            if !ip.is_empty() {
                return ip.to_string();
            }
        }
    }
    if remote_addr.is_empty() {
        "unknown".to_string()
    } else {
        remote_addr.to_string()
    }
}

/// Static limits exposed to the frontend via `GET /api/chat/status`.
#[derive(Debug, Clone, Serialize)]
pub struct ChatLimits {
    #[serde(rename = "perMinute")]
    pub per_minute: u32,
    pub daily: u32,
    #[serde(rename = "maxMessageChars")]
    pub max_message_chars: u32,
    #[serde(rename = "maxCodeChars")]
    pub max_code_chars: u32,
    #[serde(rename = "maxBodyBytes")]
    pub max_body_bytes: u32,
}

/// Returned when a client exceeds chat quotas.
#[derive(Debug, Clone)]
pub struct RateLimitError {
    pub message: String,
    pub retry_after: u32,
}

/// Per-client minute + daily counters for `POST /api/chat`.
pub struct ChatLimiter {
    pub limits: ChatLimits,
    minute: Mutex<HashMap<String, (i64, u32)>>,
    daily: Mutex<HashMap<String, (i64, u32)>>,
}

impl ChatLimiter {
    pub fn from_env() -> Self {
        Self {
            limits: ChatLimits {
                per_minute: env_int("CHAT_RATE_PER_MIN", 5),
                daily: env_int("CHAT_RATE_DAILY", 50),
                max_message_chars: env_int("CHAT_MAX_MESSAGE_CHARS", 4000),
                max_code_chars: env_int("CHAT_MAX_CODE_CHARS", 16000),
                max_body_bytes: env_int("CHAT_MAX_BODY_BYTES", 65536),
            },
            minute: Mutex::new(HashMap::new()),
            daily: Mutex::new(HashMap::new()),
        }
    }

    pub fn check(&self, client: &str) -> Option<RateLimitError> {
        if self.limits.per_minute == 0 && self.limits.daily == 0 {
            return None;
        }

        let now = unix_now();
        let minute_bucket = now / 60;
        let day_bucket = now / 86400;
        let key = if client.is_empty() { "unknown" } else { client };

        if self.limits.per_minute > 0 {
            let mut minute = self.minute.lock().expect("minute lock");
            self.prune_minute(&mut minute, minute_bucket, now);
            let (bucket, count) = minute.get(key).copied().unwrap_or((minute_bucket, 0));
            let (bucket, count) = if bucket == minute_bucket {
                (bucket, count)
            } else {
                (minute_bucket, 0)
            };
            if count >= self.limits.per_minute {
                let retry = (60 - (now % 60)).max(1) as u32;
                return Some(RateLimitError {
                    message: format!(
                        "rate limit exceeded: {} requests per minute",
                        self.limits.per_minute
                    ),
                    retry_after: retry,
                });
            }
            minute.insert(key.to_string(), (bucket, count + 1));
        }

        if self.limits.daily > 0 {
            let mut daily = self.daily.lock().expect("daily lock");
            self.prune_daily(&mut daily, day_bucket, now);
            let (bucket, count) = daily.get(key).copied().unwrap_or((day_bucket, 0));
            let (bucket, count) = if bucket == day_bucket {
                (bucket, count)
            } else {
                (day_bucket, 0)
            };
            if count >= self.limits.daily {
                let retry = (86400 - (now % 86400)).max(1) as u32;
                return Some(RateLimitError {
                    message: format!(
                        "rate limit exceeded: {} requests per day",
                        self.limits.daily
                    ),
                    retry_after: retry,
                });
            }
            daily.insert(key.to_string(), (bucket, count + 1));
        }

        None
    }

    fn prune_minute(&self, minute: &mut HashMap<String, (i64, u32)>, bucket: i64, now: i64) {
        if minute.len() > 10_000 {
            minute.retain(|_, (b, _)| *b == bucket);
        }
        if now % 600 == 0 {
            let prev = bucket - 2;
            minute.retain(|_, (b, _)| *b >= prev);
        }
    }

    fn prune_daily(&self, daily: &mut HashMap<String, (i64, u32)>, bucket: i64, now: i64) {
        if daily.len() > 10_000 {
            daily.retain(|_, (b, _)| *b == bucket);
        }
        if now % 600 == 0 {
            let prev = bucket - 2;
            daily.retain(|_, (b, _)| *b >= prev);
        }
    }
}

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
