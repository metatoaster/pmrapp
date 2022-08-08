use std::net::SocketAddr;
use axum::{Json,
    response::Html,
    response::Response, response::IntoResponse,
    routing::get, Router};
use serde::Serialize;
use client::sauron::Render;
pub use client::sauron;

mod page;

const DEFAULT_PORT: u16 = 9380;

#[tokio::main]
async fn main() {
    let route = Router::new()
        .route("/", get(index_root))
        .route("/api/", get(api_root))
        ;
    let port = DEFAULT_PORT;
    let socket: SocketAddr = ([0, 0, 0, 0], port).into();

    println!("serving at: http://{}", socket);
    axum::Server::bind(&socket)
        .serve(route.into_make_service())
        .await
        .unwrap();
}


async fn index_root() -> Response {
    let index = page::index().render_to_string();
    Html(index).into_response()
}

#[derive(Serialize)]
struct Page {
    name: String,
}


async fn api_root() -> Response {
    let resp = Page { name: "index".to_string() };
    Json(resp).into_response()
}
