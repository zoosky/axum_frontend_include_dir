use std::path::PathBuf;
use axum::{
    http::{StatusCode, header},
    response::{Response, IntoResponse},
    Router,
    routing::get,
    extract::Path,
    body::Body
};

use include_dir::{include_dir, Dir, File};
use mime_guess::{Mime, mime};
use time::Duration;

const ROOT: &str = "";
const DEFAULT_FILES: [&str; 2] = ["index.html", "index.htm"];
const NOT_FOUND: &str = "404.html";

static FRONTEND_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/frontend/dist");

async fn serve_asset(path: Option<Path<String>>) -> impl IntoResponse {
    let serve_file = |file: &File, mime_type: Option<Mime>, cache: Duration, code: Option<StatusCode>| {
        Response::builder()
            .status(code.unwrap_or(StatusCode::OK))
            .header(header::CONTENT_TYPE, mime_type.unwrap_or(mime::TEXT_HTML).to_string())
            .header(header::CACHE_CONTROL, format!("max-age={}", cache.as_seconds_f32()))
            .body(Body::from(file.contents().to_owned()))
            .unwrap()
    };

    let serve_not_found = || {
        match FRONTEND_DIR.get_file(NOT_FOUND) {
            Some(file) => serve_file(file, None, Duration::ZERO, Some(StatusCode::NOT_FOUND)),
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("File Not Found"))
                .unwrap()
        }
    };

    let serve_default = |path: &str| {
        for default_file in DEFAULT_FILES.iter() {
            let default_file_path = PathBuf::from(path).join(default_file);

            if FRONTEND_DIR.get_file(default_file_path.clone()).is_some() {
                return serve_file(
                    FRONTEND_DIR.get_file(default_file_path).unwrap(),
                    None,
                    Duration::ZERO,
                    None,
                );
            }
        }

        serve_not_found()
    };

    match path {
        Some(Path(path)) => {
            if path == ROOT {
                return serve_default(&path);
            }

            FRONTEND_DIR.get_file(&path).map_or_else(
                || {
                    match FRONTEND_DIR.get_dir(&path) {
                        Some(_) => serve_default(&path),
                        None => serve_not_found()
                    }
                },
                |file| {
                    let mime_type = mime_guess::from_path(PathBuf::from(path.clone())).first_or_octet_stream();
                    let cache = if mime_type == mime::TEXT_HTML {
                        Duration::ZERO
                    } else {
                        Duration::days(365)
                    };

                    serve_file(file, Some(mime_type), cache, None)
                },
            )
        }
        None => serve_not_found()
    }
}

pub(crate) fn router() -> Router {
    Router::new()
        .route("/", get(|| async { serve_asset(Some(Path(String::from(ROOT)))).await }))
        .route("/*path", get(|path| async { serve_asset(Some(path)).await }))
}