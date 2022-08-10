use axum::{
    extract::{
        Extension,
        Path,
    },
    Json,
    response::{
        Html,
        IntoResponse,
        Response,
    },
    routing::get, Router
};
use pmrmodel::{
    backend::db::{
        SqliteBackend,
    },
    model::workspace::{
        WorkspaceBackend,
        JsonWorkspaceRecords,
        stream_workspace_records_as_json,
    },
    repo::git::{
        GitPmrAccessor,
        GitResultSet,
        object_to_info,
    }
};
use serde::Serialize;
use std::{
    net::SocketAddr,
    sync::Arc,
    path::PathBuf,
};
use tower::ServiceBuilder;

use crate::config::Config;
use client::sauron::Render;
pub use client::sauron;

mod page;
mod error;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;


#[derive(Clone)]
struct AppContext {
    config: Arc<Config>,
    backend: SqliteBackend,
}


pub async fn serve(config: Config, backend: SqliteBackend) -> anyhow::Result<()> {
    let socket: SocketAddr = ([0, 0, 0, 0], config.http_port).into();
    let app = router().layer(
        ServiceBuilder::new()
            .layer(Extension(AppContext {
                config: Arc::new(config),
                backend: backend,
            }))
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
        .route("/api/workspace", get(api_workspace))
        .route("/api/workspace/:workspace_id", get(api_workspace_pathinfo))
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

async fn api_workspace(ctx: Extension<AppContext>) -> Result<Response> {
    let records = WorkspaceBackend::list_workspaces(&ctx.backend).await?;
    Ok(Json(JsonWorkspaceRecords { workspaces: &records }).into_response())
    // stream_workspace_records_as_json(std::io::stdout(), &records)?;
}

async fn api_workspace_pathinfo(ctx: Extension<AppContext>, workspace_id: Path<i64>) -> Result<Response> {
    let workspace = match WorkspaceBackend::get_workspace_by_id(&ctx.backend, *workspace_id).await {
        Ok(workspace) => workspace,
        Err(_) => return Err(Error::NotFound),
    };
    let git_pmr_accessor = GitPmrAccessor::new(
        &ctx.backend,
        PathBuf::from(&ctx.config.pmr_git_root),
        workspace
    );

    fn json_result(git_result_set: &GitResultSet) -> Response {
        Json(object_to_info(&git_result_set.repo, &git_result_set.object)).into_response()
    }

    Ok(git_pmr_accessor.process_pathinfo(None, None, json_result).await?)
}
