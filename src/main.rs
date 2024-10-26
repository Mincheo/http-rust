use axum::{routing::get, Router};
use std::net::SocketAddr;
use axum::extract::Query;
use axum::response::Html;
use serde::Deserialize;

#[derive(Deserialize)]
struct Name {
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Listen on {}", addr);

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn hello_handler(Query(params): Query<Name>) -> Html<String> {
    let html_content = format!("Hello, {}", params.name);
    Html(html_content)
}
