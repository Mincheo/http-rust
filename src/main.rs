use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello/:name", get(hello_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Listen on {}", addr);

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn hello_handler(axum::extract::Path(name): axum::extract::Path<String>) -> String {
    format!("Hello, {}", name)
}
