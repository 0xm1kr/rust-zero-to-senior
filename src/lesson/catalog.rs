//! The ordered list of lessons shown in the sidebar. Order matters because
//! the UI's "Next" / "Prev" buttons walk the slice, and the sidebar groups
//! by category in first-appearance order.
//!
//! The catalog is assembled from per-category slices living in their own
//! `lessons_<category>.rs` files. This file is the single place that
//! decides the pedagogical order; to add a new section, define a new
//! module and append its name here in the right slot.

use super::Lesson;

use super::lessons_algorithms::lessons as algorithms;
use super::lessons_basics::lessons as basics;
use super::lessons_concurrency::lessons as concurrency;
use super::lessons_control_flow::lessons as control_flow;
use super::lessons_data_structures::lessons as data_structures;
use super::lessons_ecosystem::lessons as ecosystem;
use super::lessons_errors::lessons as errors;
use super::lessons_interview_prep::lessons as interview_prep;
use super::lessons_lifetimes::lessons as lifetimes;
use super::lessons_memory::lessons as memory;
use super::lessons_ownership::lessons as ownership;
use super::lessons_pitfalls::lessons as pitfalls;
use super::lessons_stdlib::lessons as stdlib;
use super::lessons_tooling::lessons as tooling;
use super::lessons_traits_generics::lessons as traits_generics;
use super::lessons_web::lessons as web;

/// Returns the ordered curriculum, flattened.
pub fn catalog() -> Vec<Lesson> {
    let groups: Vec<Vec<Lesson>> = vec![
        // ── Foundations ─────────────────────────────────────────────
        basics(),
        control_flow(),
        ownership(),
        data_structures(),
        traits_generics(),
        errors(),
        concurrency(),
        tooling(),
        stdlib(),
        web(),
        ecosystem(),
        // ── Senior / Interview Track ────────────────────────────────
        lifetimes(),
        memory(),
        pitfalls(),
        algorithms(),
        interview_prep(),
    ];

    let total: usize = groups.iter().map(Vec::len).sum();
    let mut out = Vec::with_capacity(total);
    for g in groups {
        out.extend(g);
    }
    out
}
