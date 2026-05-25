use anyhow::anyhow;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::httpx::post_json;
use super::provider::{env_or_default, Message, Provider};

/// Provider implementation for OpenAI's chat-completions API.
pub struct OpenAI {
    api_key: String,
    model: String,
}

impl OpenAI {
    /// Defaults to `gpt-4o-mini`; override via `OPENAI_MODEL`.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: env_or_default("OPENAI_MODEL", "gpt-4o-mini"),
        }
    }
}

#[derive(Serialize)]
struct ReqBody<'a> {
    model: &'a str,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize, Default)]
struct ChoiceMsg {
    #[serde(default)]
    content: String,
}
#[derive(Deserialize)]
struct Choice {
    #[serde(default)]
    message: ChoiceMsg,
}
#[derive(Deserialize)]
struct Resp {
    #[serde(default)]
    choices: Vec<Choice>,
}

#[async_trait]
impl Provider for OpenAI {
    fn name(&self) -> &str {
        "openai"
    }
    fn model(&self) -> &str {
        &self.model
    }

    async fn complete(&self, system: &str, history: &[Message]) -> anyhow::Result<String> {
        let mut messages = Vec::with_capacity(history.len() + 1);
        messages.push(Message {
            role: "system".to_string(),
            content: system.to_string(),
        });
        messages.extend(history.iter().cloned());

        let body = ReqBody {
            model: &self.model,
            messages,
            max_tokens: 1024,
            temperature: 0.4,
        };
        let auth = format!("Bearer {}", self.api_key);
        let headers = [("Authorization", auth.as_str())];

        let resp: Resp = post_json(
            "https://api.openai.com/v1/chat/completions",
            &headers,
            &body,
        )
        .await
        .map_err(|e| anyhow!("OpenAI {e}"))?;

        let first = resp
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("OpenAI returned no choices"))?;
        Ok(first.message.content.trim().to_string())
    }
}
