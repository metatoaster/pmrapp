use axum::{
    extract::Extension,
    Json,
    response::{
        Html,
        IntoResponse,
        Response,
    },
    routing::get,
    Router,
};
use pmrmodel::backend::db::SqliteBackend;
use serde::Serialize;
use std::{
    net::SocketAddr,
    sync::Arc,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use client::sauron::Render;
pub use client::sauron;

mod page;
mod error;
mod api;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;


#[derive(Clone)]
struct AppContext {
    config: Arc<Config>,
    backend: SqliteBackend,
}


pub async fn serve(config: Config, backend: SqliteBackend) -> anyhow::Result<()> {
    let socket: SocketAddr = ([0, 0, 0, 0], config.http_port).into();
    let app = router()
        .nest("/api/workspace/", api::workspace::router())
        .layer(
            ServiceBuilder::new()
            .layer(Extension(AppContext {
                config: Arc::new(config),
                backend: backend,
            }))
            .layer(TraceLayer::new_for_http())
        );

    println!("serving at: http://{}", socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}


fn router() -> Router {
    Router::new()
        .route("/", get(index_root))
        .route("/api/", get(api_root))
}

// placeholder thingers
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
