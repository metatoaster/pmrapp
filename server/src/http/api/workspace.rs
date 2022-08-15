use axum::{
    extract::{Extension, Path},
    Json,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use pmrmodel::{
    model::workspace::{
        WorkspaceBackend,
        JsonWorkspaceRecords,
    },
    repo::git::{
        GitPmrAccessor,
        ObjectInfo,
    }
};
use std::path::PathBuf;

use crate::http::AppContext;
use crate::http::{Error, Result};


pub fn router() -> Router {
    // XXX this is done so because we want trailing / via nest
    // See: https://github.com/tokio-rs/axum/issues/714
    // See: https://github.com/tokio-rs/axum/pull/824
    // Note comment about how routes should be working standalone
    // FIXME by axum-0.6
    Router::new()
        .route("/", get(api_workspace))
        .route(":workspace_id/",
            get(api_workspace_pathinfo_workspace_id))
        .route(":workspace_id/file/",
            get(api_workspace_pathinfo_workspace_id))
        .route(":workspace_id/file/:commit_id/*path",
            get(api_workspace_pathinfo_workspace_id_commit_id_path))
}

async fn api_workspace(ctx: Extension<AppContext>) -> Result<Response> {
    let records = WorkspaceBackend::list_workspaces(&ctx.backend).await?;
    Ok(Json(JsonWorkspaceRecords { workspaces: records }).into_response())
}

async fn api_workspace_pathinfo(
    ctx: Extension<AppContext>,
    workspace_id: i64,
    commit_id: Option<String>,
    path: Option<String>,
) -> Result<Json<ObjectInfo>> {
    let workspace = match WorkspaceBackend::get_workspace_by_id(&ctx.backend, workspace_id).await {
        Ok(workspace) => workspace,
        Err(_) => return Err(Error::NotFound),
    };
    let git_pmr_accessor = GitPmrAccessor::new(
        &ctx.backend,
        PathBuf::from(&ctx.config.pmr_git_root),
        workspace
    );

    match git_pmr_accessor.process_pathinfo(
        commit_id.as_deref(),
        path.as_deref(),
        |result| <Option<ObjectInfo>>::from(result)
    ).await {
        Ok(result) => match result {
            Some(result) => Ok(Json(result)),
            // tags and other random nodes are not part of the path
            // TODO subrepo/tree vs. embedded workspace links to be
            // redirected
            None => Err(Error::NotFound),
        }
        Err(e) => {
            // TODO log the URI triggering these messages?
            log::info!("git_pmr_accessor.process_pathinfo error: {:?}", e);
            Err(Error::NotFound)
        }
    }
}

async fn api_workspace_pathinfo_workspace_id(
    ctx: Extension<AppContext>,
    Path(workspace_id): Path<i64>,
) -> Result<Json<ObjectInfo>> {
    api_workspace_pathinfo(ctx, workspace_id, None, None).await
}

async fn api_workspace_pathinfo_workspace_id_commit_id_path(
    ctx: Extension<AppContext>,
    Path((workspace_id, commit_id, path)): Path<(i64, Option<String>, Option<String>)>,
) -> Result<Json<ObjectInfo>> {
    api_workspace_pathinfo(ctx, workspace_id, commit_id, path).await
}

