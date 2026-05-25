use std::sync::Arc;

use anyhow::anyhow;
use once_cell::sync::Lazy;
use regex_lite::Regex;
use serde::Serialize;

use crate::lesson::{Lesson, Repository};

use super::provider::{Message, Provider};

// We use a tiny regex-lite-free strip, see below.

/// The chat use-case orchestrator. Given the lesson context and the current
/// conversation, it builds a system prompt and asks the [`Provider`] for a
/// reply. It has no opinion on HTTP, JSON, or transport — those live in
/// the `api` module.
pub struct Service {
    provider: Option<Arc<dyn Provider>>,
    lessons: Arc<dyn Repository>,
}

impl Service {
    /// Wires a Service. `provider` may be `None` — in that case
    /// [`Service::status`] reports unavailable and [`Service::reply`]
    /// returns a configuration error.
    pub fn new(provider: Option<Arc<dyn Provider>>, lessons: Arc<dyn Repository>) -> Self {
        Self { provider, lessons }
    }

    /// Reports whether chat is available and which provider/model is in use.
    pub fn status(&self) -> Status {
        match &self.provider {
            None => Status {
                available: false,
                provider: String::new(),
                model: String::new(),
                hint: Some(
                    "Set GEMINI_API_KEY (or ANTHROPIC_API_KEY / OPENAI_API_KEY) in your environment or .env and restart the server.".to_string(),
                ),
            },
            Some(p) => Status {
                available: true,
                provider: p.name().to_string(),
                model: p.model().to_string(),
                hint: None,
            },
        }
    }

    /// Produces the next assistant message for the given conversation.
    ///
    /// `lesson_id` + `current_code` supply context: the student's lesson
    /// and whatever they currently have in the editor. `history` is the
    /// full conversation so far (the last entry should be a user message).
    pub async fn reply(
        &self,
        lesson_id: &str,
        current_code: &str,
        history: &[Message],
    ) -> anyhow::Result<Message> {
        let Some(provider) = self.provider.as_ref() else {
            return Err(anyhow!("no LLM API key configured"));
        };

        let lesson = self.lessons.by_id(lesson_id);
        let system = build_system_prompt(lesson, current_code);

        let reply = provider.complete(&system, history).await?;
        Ok(Message {
            role: "assistant".to_string(),
            content: reply,
        })
    }
}

/// Mirrors the shape returned by `GET /api/chat/status`.
#[derive(Debug, Clone, Serialize)]
pub struct Status {
    pub available: bool,
    pub provider: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// Assembles the per-request system message. Pure function, kept private
/// to the module because it's an implementation detail.
fn build_system_prompt(l: Option<&Lesson>, current_code: &str) -> String {
    let mut s = String::with_capacity(2048);
    s.push_str(
        "You are a friendly, concise Rust programming tutor helping an experienced engineer learn Rust through an interactive tutorial app. ",
    );
    s.push_str(
        "Answer questions about Rust syntax, idioms, the borrow checker, the standard library, async/await, and the ecosystem. ",
    );
    s.push_str(
        "Prefer short, direct answers with small runnable code examples. Always wrap Rust code in ```rust fenced blocks. ",
    );
    s.push_str("If the student appears confused or wrong about something, gently correct them. ");
    s.push_str("If a question is unrelated to Rust, briefly redirect to the lesson.\n\n");

    if let Some(l) = l {
        s.push_str(&format!(
            "## Current lesson\n\nTitle: {}\nCategory: {}\n\n",
            l.title, l.category
        ));
        let desc = strip_html(l.description);
        if !desc.is_empty() {
            s.push_str("Description:\n");
            s.push_str(&desc);
            s.push_str("\n\n");
        }
        if !l.code.is_empty() {
            s.push_str("Reference code shipped with this lesson:\n```rust\n");
            s.push_str(l.code);
            s.push_str("\n```\n\n");
        }
        if !l.notes.is_empty() {
            s.push_str("Key takeaways:\n");
            for n in &l.notes {
                s.push_str("- ");
                s.push_str(n);
                s.push('\n');
            }
            s.push('\n');
        }
    }

    let trimmed = current_code.trim();
    let reference_code = l.map(|l| l.code.trim()).unwrap_or("");
    if !trimmed.is_empty() && trimmed != reference_code {
        s.push_str("The student's CURRENT editor contents (may differ from the reference code):\n```rust\n");
        s.push_str(current_code);
        s.push_str("\n```\n");
    }
    s
}

// We deliberately don't pull in a full HTML parser; the lesson descriptions
// are small and the strip just needs to remove tags + decode a handful of
// common entities for use in the system prompt. Using a tiny hand-rolled
// state machine here keeps deps small (and is itself a teaching example).

static HTML_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"<[^>]+>").expect("compile"));
static TRIPLE_NL: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n\n\n+").expect("compile"));

fn strip_html(s: &str) -> String {
    let mut out = HTML_TAG.replace_all(s, "").to_string();
    for (from, to) in [
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&amp;", "&"),
        ("&quot;", "\""),
        ("&#39;", "'"),
        ("&nbsp;", " "),
    ] {
        if out.contains(from) {
            out = out.replace(from, to);
        }
    }
    out = TRIPLE_NL.replace_all(&out, "\n\n").to_string();
    out.trim().to_string()
}
