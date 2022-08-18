use pmrmodel_base::workspace::{
    JsonWorkspaceRecords,
    // WorkspaceRecord,
};
use pmrmodel_base::git::ObjectInfo;
use crate::error::ServerError;

pub async fn request_get_json<T: serde::de::DeserializeOwned>(
    url: &str,
) -> Result<T, ServerError> {
    log::trace!("request_get_json: {}", url);
    let response = reqwest::get(url).await?;
    Ok(response.json::<T>().await?)
}

pub async fn get_workspace_listing() -> Result<JsonWorkspaceRecords, ServerError> {
    let url = format!("{}/api/workspace/", sauron::window().location().origin().expect("must have location"));
    Ok(request_get_json::<JsonWorkspaceRecords>(&url).await?)
}

pub async fn get_workspace(workspace_id: i64) -> Result<ObjectInfo, ServerError> {
    let url = format!("{}/api/workspace/{}/", sauron::window().location().origin().expect("must have location"), workspace_id);
    Ok(request_get_json::<ObjectInfo>(&url).await?)
}
