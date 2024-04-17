use axum::{Router, routing::get};

pub(crate) fn router() -> Router {
    Router::new().route("/health", get(|| async { "ok" }))
}