use std::env;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::anthropic::Anthropic;
use super::gemini::Gemini;
use super::openai::OpenAI;

/// The wire shape exchanged with the frontend AND fed into each provider
/// after a small role translation. `role` is `"user"` or `"assistant"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// The LLM adapter port. Implementations are interchangeable — the
/// [`super::Service`] knows nothing about HTTP, vendors, or model strings.
#[async_trait]
pub trait Provider: Send + Sync {
    /// e.g. `"google"`, `"anthropic"`, `"openai"`
    fn name(&self) -> &str;
    /// e.g. `"gemini-2.5-flash"`
    fn model(&self) -> &str;
    async fn complete(&self, system: &str, history: &[Message]) -> anyhow::Result<String>;
}

/// Returns the highest-priority provider whose API key is set in the
/// environment, or `None` if none is configured.
///
/// Priority order: Google → Anthropic → OpenAI.
pub fn select_from_env() -> Option<Arc<dyn Provider>> {
    if let Ok(key) = env::var("GEMINI_API_KEY") {
        if !key.is_empty() {
            return Some(Arc::new(Gemini::new(key)));
        }
    }
    if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
        if !key.is_empty() {
            return Some(Arc::new(Anthropic::new(key)));
        }
    }
    if let Ok(key) = env::var("OPENAI_API_KEY") {
        if !key.is_empty() {
            return Some(Arc::new(OpenAI::new(key)));
        }
    }
    None
}

/// Tiny shared helper so each provider's model lookup is a one-liner.
pub(crate) fn env_or_default(key: &str, def: &str) -> String {
    env::var(key)
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| def.to_string())
}
