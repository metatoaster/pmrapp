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
        GitResultSet,
        object_to_info,
    }
};
use std::path::PathBuf;

use crate::http::AppContext;
use crate::http::{Error, Result};


pub fn router() -> Router {
    Router::new()
        // TODO remove /api/workspace, set mount point
        .route("/api/workspace", get(api_workspace))
        .route("/api/workspace/:workspace_id/",
            get(api_workspace_pathinfo_workspace_id))
        .route("/api/workspace/:workspace_id/file/",
            get(api_workspace_pathinfo_workspace_id))
        .route("/api/workspace/:workspace_id/file/:commit_id/*path",
            get(api_workspace_pathinfo_workspace_id_commit_id_path))
}

async fn api_workspace(ctx: Extension<AppContext>) -> Result<Response> {
    let records = WorkspaceBackend::list_workspaces(&ctx.backend).await?;
    Ok(Json(JsonWorkspaceRecords { workspaces: &records }).into_response())
    // stream_workspace_records_as_json(std::io::stdout(), &records)?;
}

async fn api_workspace_pathinfo(
    ctx: Extension<AppContext>,
    workspace_id: i64,
    commit_id: Option<String>,
    path: Option<String>,
) -> Result<Response> {
    let workspace = match WorkspaceBackend::get_workspace_by_id(&ctx.backend, workspace_id).await {
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

    match git_pmr_accessor.process_pathinfo(
        commit_id.as_deref(),
        path.as_deref(),
        json_result
    ).await {
        Ok(result) => Ok(result),
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
) -> Result<Response> {
    api_workspace_pathinfo(ctx, workspace_id, None, None).await
}

async fn api_workspace_pathinfo_workspace_id_commit_id_path(
    ctx: Extension<AppContext>,
    Path((workspace_id, commit_id, path)): Path<(i64, Option<String>, Option<String>)>,
) -> Result<Response> {
    api_workspace_pathinfo(ctx, workspace_id, commit_id, path).await
}

