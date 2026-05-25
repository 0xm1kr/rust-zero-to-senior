//! The LLM-backed chat domain. It exposes a [`Service`] that orchestrates
//! a request (lesson context + history) against any [`Provider`], plus
//! three concrete providers: Gemini, Anthropic, OpenAI.

mod anthropic;
mod gemini;
mod httpx;
mod openai;
mod provider;
mod service;

#[allow(unused_imports)]
pub use provider::{select_from_env, Message, Provider};
#[allow(unused_imports)]
pub use service::{Service, Status};
