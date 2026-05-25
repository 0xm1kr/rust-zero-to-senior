use anyhow::anyhow;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::httpx::post_json;
use super::provider::{env_or_default, Message, Provider};

/// Provider implementation for Google's Generative Language API (Gemini).
pub struct Gemini {
    api_key: String,
    model: String,
}

impl Gemini {
    /// Build a provider backed by Google's Gemini API.
    /// Defaults to `gemini-2.5-flash`; override via `GEMINI_MODEL`.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: env_or_default("GEMINI_MODEL", "gemini-2.5-flash"),
        }
    }
}

#[derive(Serialize)]
struct Part<'a> {
    text: &'a str,
}

#[derive(Serialize)]
struct Content<'a> {
    role: &'a str,
    parts: [Part<'a>; 1],
}

#[derive(Serialize)]
struct SystemInstruction<'a> {
    parts: [Part<'a>; 1],
}

#[derive(Serialize)]
struct GenerationConfig {
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
    temperature: f32,
}

#[derive(Serialize)]
struct ReqBody<'a> {
    contents: Vec<Content<'a>>,
    #[serde(rename = "systemInstruction")]
    system_instruction: SystemInstruction<'a>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Deserialize)]
struct RespPart {
    #[serde(default)]
    text: String,
}
#[derive(Deserialize, Default)]
struct RespContent {
    #[serde(default)]
    parts: Vec<RespPart>,
}
#[derive(Deserialize)]
struct Candidate {
    #[serde(default)]
    content: RespContent,
    #[serde(rename = "finishReason", default)]
    finish_reason: String,
}
#[derive(Deserialize, Default)]
struct PromptFeedback {
    #[serde(rename = "blockReason", default)]
    block_reason: String,
}
#[derive(Deserialize)]
struct Resp {
    #[serde(default)]
    candidates: Vec<Candidate>,
    #[serde(default, rename = "promptFeedback")]
    prompt_feedback: PromptFeedback,
}

#[async_trait]
impl Provider for Gemini {
    fn name(&self) -> &str {
        "google"
    }
    fn model(&self) -> &str {
        &self.model
    }

    async fn complete(&self, system: &str, history: &[Message]) -> anyhow::Result<String> {
        let contents: Vec<Content<'_>> = history
            .iter()
            .map(|m| {
                let role = if m.role == "assistant" {
                    "model"
                } else {
                    m.role.as_str()
                };
                Content {
                    role,
                    parts: [Part {
                        text: m.content.as_str(),
                    }],
                }
            })
            .collect();

        let body = ReqBody {
            contents,
            system_instruction: SystemInstruction {
                parts: [Part { text: system }],
            },
            generation_config: GenerationConfig {
                max_output_tokens: 1024,
                temperature: 0.4,
            },
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
            self.model
        );
        let headers = [("x-goog-api-key", self.api_key.as_str())];
        let resp: Resp = post_json(&url, &headers, &body)
            .await
            .map_err(|e| anyhow!("Gemini {e}"))?;

        if !resp.prompt_feedback.block_reason.is_empty() {
            return Err(anyhow!(
                "Gemini blocked the prompt: {}",
                resp.prompt_feedback.block_reason
            ));
        }
        let Some(first) = resp.candidates.into_iter().next() else {
            return Err(anyhow!("Gemini returned no candidates"));
        };

        let mut texts: Vec<String> = first
            .content
            .parts
            .into_iter()
            .map(|p| p.text)
            .filter(|t| !t.is_empty())
            .collect();
        if texts.is_empty() {
            return Err(anyhow!(
                "Gemini returned empty content (finish reason: {})",
                first.finish_reason
            ));
        }
        if texts.len() == 1 {
            return Ok(texts.remove(0).trim().to_string());
        }
        Ok(texts.join("\n\n").trim().to_string())
    }
}
