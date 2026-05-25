use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use super::AppState;

/// Returns the catalog as an ordered list of summary records. The frontend
/// uses this to build the sidebar without paying for description/code/notes
/// payloads it doesn't yet need.
pub async fn list(State(s): State<AppState>) -> impl IntoResponse {
    let all = s.lessons.all();
    let out: Vec<_> = all.iter().map(|l| l.summary()).collect();
    Json(out)
}

/// Returns one full Lesson (description, starter code, notes) keyed by
/// the trailing path segment. Returns 404 if no lesson matches.
pub async fn by_id(State(s): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match s.lessons.by_id(&id) {
        Some(l) => Json(l.clone()).into_response(),
        None => (StatusCode::NOT_FOUND, Json(json!({"error": "not found"}))).into_response(),
    }
}
