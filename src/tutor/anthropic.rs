use anyhow::anyhow;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::httpx::post_json;
use super::provider::{env_or_default, Message, Provider};

/// Provider implementation for Anthropic's Messages API (Claude).
pub struct Anthropic {
    api_key: String,
    model: String,
}

impl Anthropic {
    /// Defaults to `claude-haiku-4-5`; override via `ANTHROPIC_MODEL`.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: env_or_default("ANTHROPIC_MODEL", "claude-haiku-4-5"),
        }
    }
}

#[derive(Serialize)]
struct ReqBody<'a> {
    model: &'a str,
    system: &'a str,
    messages: &'a [Message],
    max_tokens: u32,
}

#[derive(Deserialize)]
struct Block {
    #[serde(rename = "type", default)]
    kind: String,
    #[serde(default)]
    text: String,
}

#[derive(Deserialize)]
struct Resp {
    #[serde(default)]
    content: Vec<Block>,
}

#[async_trait]
impl Provider for Anthropic {
    fn name(&self) -> &str {
        "anthropic"
    }
    fn model(&self) -> &str {
        &self.model
    }

    async fn complete(&self, system: &str, history: &[Message]) -> anyhow::Result<String> {
        let body = ReqBody {
            model: &self.model,
            system,
            messages: history,
            max_tokens: 1024,
        };
        let headers = [
            ("x-api-key", self.api_key.as_str()),
            ("anthropic-version", "2023-06-01"),
        ];
        let resp: Resp = post_json("https://api.anthropic.com/v1/messages", &headers, &body)
            .await
            .map_err(|e| anyhow!("Anthropic {e}"))?;

        let parts: Vec<String> = resp
            .content
            .into_iter()
            .filter(|b| b.kind == "text")
            .map(|b| b.text)
            .collect();
        if parts.is_empty() {
            return Err(anyhow!("Anthropic returned no text content"));
        }
        Ok(parts.join("\n\n").trim().to_string())
    }
}
