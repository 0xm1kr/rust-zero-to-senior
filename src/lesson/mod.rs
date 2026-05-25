//! The lesson domain.
//!
//! It exposes the [`Lesson`] value object, a lightweight [`Summary`]
//! projection for the sidebar, and a [`Repository`] port so callers don't
//! have to care whether lessons are loaded from memory, disk, or a
//! database. The static curriculum lives in this module as well
//! (`catalog.rs` + `lessons_*.rs`).

use std::collections::HashMap;
use std::sync::Arc;

use serde::Serialize;

mod catalog;
mod lessons_algorithms;
mod lessons_basics;
mod lessons_concurrency;
mod lessons_control_flow;
mod lessons_data_structures;
mod lessons_ecosystem;
mod lessons_errors;
mod lessons_interview_prep;
mod lessons_lifetimes;
mod lessons_memory;
mod lessons_ownership;
mod lessons_pitfalls;
mod lessons_stdlib;
mod lessons_tooling;
mod lessons_traits_generics;
mod lessons_web;

pub use catalog::catalog;

/// One tutorial unit shown by the UI.
///
/// `description` is raw HTML so the curriculum can use headings, lists,
/// links, tables, and inline `<code>` without pulling in a markdown
/// dependency. `code` is the initial buffer rendered into the in-browser
/// editor when the lesson is opened. `notes` appear in the
/// "Key takeaways" callout below the editor.
#[derive(Debug, Clone, Serialize)]
pub struct Lesson {
    pub id: &'static str,
    pub category: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub code: &'static str,
    pub notes: Vec<&'static str>,
}

/// The lightweight projection used by the sidebar. Stripping out
/// description/code/notes keeps the /api/lessons response small even as
/// the curriculum grows; the full Lesson is fetched lazily on selection.
#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub id: &'static str,
    pub title: &'static str,
    pub category: &'static str,
}

impl Lesson {
    pub fn summary(&self) -> Summary {
        Summary {
            id: self.id,
            title: self.title,
            category: self.category,
        }
    }
}

/// The access port for lessons.
///
/// Today there's a single implementation ([`InMemoryRepository`]); tomorrow
/// a `FileRepository` or `SqlRepository` could swap in without touching
/// the `api` or `tutor` modules.
pub trait Repository: Send + Sync {
    fn all(&self) -> &[Lesson];
    fn by_id(&self, id: &str) -> Option<&Lesson>;
}

/// Serves lessons from an in-process Vec. Lookups by id are O(1) via a
/// precomputed index built at construction time.
pub struct InMemoryRepository {
    items: Vec<Lesson>,
    index: HashMap<&'static str, usize>,
}

impl InMemoryRepository {
    pub fn new(items: Vec<Lesson>) -> Self {
        let mut index = HashMap::with_capacity(items.len());
        for (i, l) in items.iter().enumerate() {
            index.insert(l.id, i);
        }
        Self { items, index }
    }
}

impl Repository for InMemoryRepository {
    fn all(&self) -> &[Lesson] {
        &self.items
    }
    fn by_id(&self, id: &str) -> Option<&Lesson> {
        self.index.get(id).map(|&i| &self.items[i])
    }
}

// Blanket impl so `Arc<R>` is itself a Repository — lets us pass a single
// `Arc<InMemoryRepository>` everywhere without dealing with `&dyn Repository`
// lifetimes in the handlers.
impl<R: Repository + ?Sized> Repository for Arc<R> {
    fn all(&self) -> &[Lesson] {
        (**self).all()
    }
    fn by_id(&self, id: &str) -> Option<&Lesson> {
        (**self).by_id(id)
    }
}
