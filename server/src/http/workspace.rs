use axum::{
    extract::{Extension, Path},
    Json,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use crate::http::{
    api,
    AppContext,
    Error,
    Html,
    page,
};
use client::App;
use client::sauron::Render;


pub fn router() -> Router {
    Router::new()
        .route("/", get(render_workspace_listing))
        .route("/:workspace_id/", get(render_workspace))
}

async fn render_workspace_listing(ctx: Extension<AppContext>) -> Response {
    match api::workspace::api_workspace(ctx).await {
        Ok(Json(workspace_listing)) => {
            let app = App::with_workspace_listing(workspace_listing);
            let content = page::index(&app).render_to_string();
            Html(content).into_response()
        },
        Err(e) => Error::from(e).into_response()
    }
}

async fn render_workspace(
    ctx: Extension<AppContext>,
    path: Path<i64>,
) -> Response {
    let workspace_id = path.0;
    match api::workspace::api_workspace_top_ssr(ctx, path).await {
        Ok((record, path_info)) => {
            let app = App::with_workspace_top(workspace_id, record, path_info);
            let content = page::index(&app).render_to_string();
            Html(content).into_response()
        },
        Err(e) => Error::from(e).into_response()
    }
}
