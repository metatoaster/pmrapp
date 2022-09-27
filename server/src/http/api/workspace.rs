use axum::{
    extract::{Extension, Path},
    Json,
    routing::get,
    Router,
};
use pmrmodel::model::workspace::WorkspaceBackend;
use pmrmodel::repo::git::{
    WorkspaceGitResultSet,
    GitPmrAccessor,
};
use pmrmodel_base::merged::WorkspacePathInfo;
use pmrmodel_base::workspace::JsonWorkspaceRecords;
use std::path::PathBuf;

use client::model::JsonWorkspaceRecord;
use crate::http::AppContext;
use crate::http::{Error, Result};


pub fn router() -> Router {
    Router::new()
        .route("/", get(api_workspace))
        .route("/:workspace_id/",
            get(api_workspace_top))
        .route("/:workspace_id/file/",
            get(api_workspace_pathinfo_workspace_id))
        .route("/:workspace_id/file/:commit_id/",
            get(api_workspace_pathinfo_workspace_id_commit_id))
        .route("/:workspace_id/file/:commit_id/*path",
            get(api_workspace_pathinfo_workspace_id_commit_id_path))
}

pub async fn api_workspace(ctx: Extension<AppContext>) -> Result<Json<JsonWorkspaceRecords>> {
    let records = WorkspaceBackend::list_workspaces(&ctx.backend).await?;
    Ok(Json(JsonWorkspaceRecords { workspaces: records }))
}

pub async fn api_workspace_top(
    ctx: Extension<AppContext>,
    Path(workspace_id): Path<i64>,
) -> Result<Json<JsonWorkspaceRecord>> {
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
        None,
        None,
        |_, result| { format!("{}", result.commit.id()) }
    ).await {
        Ok(commit_id) => Ok(Json(JsonWorkspaceRecord {
            workspace: git_pmr_accessor.workspace,
            head_commit: Some(commit_id),
        })),
        Err(_) => Ok(Json(JsonWorkspaceRecord {
            workspace: git_pmr_accessor.workspace,
            head_commit: None,
        })),
    }
}

pub async fn api_workspace_top_ssr(
    ctx: Extension<AppContext>,
    Path(workspace_id): Path<i64>,
) -> Result<(JsonWorkspaceRecord, Option<WorkspacePathInfo>)> {
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
        None,
        None,
        |git_pmr_accessor, result| (
            format!("{}", result.commit.id()),
            <WorkspacePathInfo>::from(
                &WorkspaceGitResultSet::new(
                    &git_pmr_accessor.workspace,
                    &result,
                )
            ),
        )
    ).await {
        Ok((commit_id, path_info)) => Ok((
            JsonWorkspaceRecord {
                workspace: git_pmr_accessor.workspace,
                head_commit: Some(commit_id),
            },
            Some(path_info),
        )),
        Err(_) => Ok((
            JsonWorkspaceRecord {
                workspace: git_pmr_accessor.workspace,
                head_commit: None,
            },
            None,
        )),
    }
}

async fn api_workspace_pathinfo(
    ctx: Extension<AppContext>,
    workspace_id: i64,
    commit_id: Option<String>,
    path: Option<String>,
) -> Result<Json<WorkspacePathInfo>> {
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
        |git_pmr_accessor, result| <WorkspacePathInfo>::from(
            &WorkspaceGitResultSet::new(
                &git_pmr_accessor.workspace,
                &result,
            )
        )
    ).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            // TODO log the URI triggering these messages?
            log::info!("git_pmr_accessor.process_pathinfo error: {:?}", e);
            Err(Error::NotFound)
        }
    }
}

pub async fn api_workspace_pathinfo_workspace_id(
    ctx: Extension<AppContext>,
    Path(workspace_id): Path<i64>,
) -> Result<Json<WorkspacePathInfo>> {
    api_workspace_pathinfo(ctx, workspace_id, None, None).await
}

pub async fn api_workspace_pathinfo_workspace_id_commit_id(
    ctx: Extension<AppContext>,
    Path((workspace_id, commit_id)): Path<(i64, String)>,
) -> Result<Json<WorkspacePathInfo>> {
    api_workspace_pathinfo(ctx, workspace_id, Some(commit_id), None).await
}


pub async fn api_workspace_pathinfo_workspace_id_commit_id_path(
    ctx: Extension<AppContext>,
    Path((workspace_id, commit_id, path)): Path<(i64, String, String)>,
) -> Result<Json<WorkspacePathInfo>> {
    api_workspace_pathinfo(ctx, workspace_id, Some(commit_id), Some(path)).await
}

