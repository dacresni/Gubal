use axum::{
    routing::get,
    Json, Router,
};
use std::{net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World!"),
    })
}

