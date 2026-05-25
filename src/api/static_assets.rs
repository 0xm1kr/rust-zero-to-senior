//! Serves the embedded `web/` directory. The frontend (HTML/CSS/JS,
//! including the vendored CodeMirror assets) is baked into the binary
//! via `rust-embed` so the final image is a single file.

use axum::body::Body;
use axum::extract::Request;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::Response;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "web/"]
struct Web;

/// axum fallback handler. Looks up the request path in the embedded asset
/// map; defaults to `index.html` for routes that don't match a file (so
/// the SPA-style hash routing keeps working on refresh).
pub async fn serve(req: Request) -> Response {
    let path = req.uri().path().trim_start_matches('/');
    let target = if path.is_empty() { "index.html" } else { path };

    if let Some(content) = Web::get(target) {
        return file_response(target, content.data.as_ref());
    }

    // Anything else: fall back to index.html (200 — single-page-app feel).
    if let Some(content) = Web::get("index.html") {
        return file_response("index.html", content.data.as_ref());
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("not found"))
        .unwrap()
}

fn file_response(path: &str, bytes: &[u8]) -> Response {
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    let mut builder = Response::builder().status(StatusCode::OK);

    if let Ok(value) = HeaderValue::from_str(mime.as_ref()) {
        builder = builder.header(header::CONTENT_TYPE, value);
    }
    builder
        .body(Body::from(bytes.to_vec()))
        .unwrap_or_else(|_| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("response build failed"))
                .unwrap()
        })
}
