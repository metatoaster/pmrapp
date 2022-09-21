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
        .route("/:workspace_id/file/:commit_id/", get(render_workspace_pathinfo_workspace_id_commit_id))
        .route("/:workspace_id/file/:commit_id/*path", get(render_workspace_pathinfo_workspace_id_commit_id_path))
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

async fn render_workspace_pathinfo_workspace_id_commit_id(
    ctx: Extension<AppContext>,
    path: Path<(i64, String)>,
) -> Response {
    let workspace_id = path.0.0;
    let commit_id = path.1.clone();
    match api::workspace::api_workspace_pathinfo_workspace_id_commit_id(ctx, path).await {
        Ok(Json(path_info)) => {
            // XXX instead of None we have an empty string for path...
            let app = App::with_workspace_pathinfo(workspace_id, commit_id, "".to_string(), path_info);
            let content = page::index(&app).render_to_string();
            Html(content).into_response()
        },
        Err(e) => Error::from(e).into_response()
    }
}

async fn render_workspace_pathinfo_workspace_id_commit_id_path(
    ctx: Extension<AppContext>,
    path: Path<(i64, String, String)>,
) -> Response {
    let workspace_id = path.0.0;
    let commit_id = path.1.clone();
    let filepath = path.2.clone();
    match api::workspace::api_workspace_pathinfo_workspace_id_commit_id_path(ctx, path).await {
        Ok(Json(path_info)) => {
            let app = App::with_workspace_pathinfo(workspace_id, commit_id, filepath, path_info);
            let content = page::index(&app).render_to_string();
            Html(content).into_response()
        },
        Err(e) => Error::from(e).into_response()
    }
}
