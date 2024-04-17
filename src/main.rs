mod routes;

use axum::Router;

use crate::routes::{healthcheck, frontend};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(healthcheck::router())
        .merge(frontend::router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}