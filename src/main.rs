use axum::Server;
use axum::{routing::get, Router};
use tokio;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hey", get(|| async { "Hello Kshitij" }));
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
