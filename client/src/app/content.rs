use crate::app;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

use pmrmodel_base::{
    workspace::{
        JsonWorkspaceRecords,
        WorkspaceRecord,
    },
    git::{
        PathInfo,
        PathObject,
        TreeEntryInfo,
    },
    merged::{
        WorkspacePathInfo,
    }
};

use crate::model::JsonWorkspaceRecord;
use crate::app::Resource;
use crate::app::Msg;

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Clone, derive_more::From,
)]
pub enum Content {
    Homepage,
    WorkspaceListing(JsonWorkspaceRecords),
    WorkspaceTop(JsonWorkspaceRecord, Option<WorkspacePathInfo>),
    WorkspacePathInfo(WorkspacePathInfo),
}


impl Content {
    pub fn view(&self) -> Node<app::Msg> {
        match self {
            Content::Homepage => {
                node! {
                    <div class="main">
                        <h1>"Physiome Model Repository"</h1>
                        <p>"Welcome to the Physiome Model Repository"</p>
                        <dl>
                            <dt><a href="/workspace/"
                                on_click=|e| {
                                    e.prevent_default();
                                    Msg::Retrieve(Resource::WorkspaceListing, "/workspace/".to_string())
                                }>"Workspace Listing"</a></dt>
                          <dd>"Listing of all workspaces in the repository."</dd>
                        </dl>
                    </div>
                }
            }
            Content::WorkspaceListing(records) => {
                node! {
                    <div class="main">
                        <h1>"Workspace Listing"</h1>
                        <div class="workspace-listing">
                        {
                            for record in &records.workspaces {
                                self.show_workspace_record_row(record)
                            }
                        }
                        </div>
                    </div>
                }
            },
            Content::WorkspaceTop(workspace_record, path_info) => {
                node! {
                    <div class="main">
                        <h1>{ text!("{}", &workspace_record.workspace.description.as_ref().unwrap_or(
                            &format!("Workspace {}", &workspace_record.workspace.id))) }</h1>
                        <dl>
                            <dt>"Git Repository URI"</dt>
                            <dd>{ text!("{}", &workspace_record.workspace.url) }</dd>
                        </dl>
                        <div class="workspace-pathinfo">
                        {
                            self.show_workspace_file_table(&path_info)
                        }
                        </div>
                    </div>
                }
            },
            Content::WorkspacePathInfo(path_info) => {
                node! {
                    <div class="main">
                        <div class="workspace-pathinfo">
                        { text!("{:?}", path_info) }
                        </div>
                    </div>
                }
            },
        }
    }

    fn show_workspace_record_row(&self, workspace_record: &WorkspaceRecord) -> Node<app::Msg> {
        let workspace_id = workspace_record.id;
        node! {
            <div>
                <div><a
                    relative
                    href=format!("/workspace/{}/", workspace_id)
                    on_click=move |e| {
                        e.prevent_default();
                        Msg::Retrieve(Resource::WorkspaceTop(workspace_id), format!("/workspace/{}/", workspace_id))
                    }
                >{ text!("Workspace: {}", workspace_id) }
                </a></div>
                <div>{ text!("{}", workspace_record.url) }</div>
                <div>{ text!("{}", workspace_record.description.as_ref().unwrap_or(&"".to_string())) }</div>
            </div>
        }
    }

    fn show_workspace_file_table(&self, path_info: &Option<WorkspacePathInfo>) -> Node<app::Msg> {
        node! {
            <table class="file-listing">
                <thead>
                    <tr>
                        <th>"Filename"</th>
                        <th>"Size"</th>
                        <th>"Date"</th>
                    </tr>
                </thead>
                {
                    self.show_workspace_file_table_body(&path_info)
                }
            </table>
        }
    }

    fn show_workspace_file_table_body(&self, path_info: &Option<WorkspacePathInfo>) -> Node<app::Msg> {
        match path_info {
            Some(path_info) => {
                match &path_info.object {
                    Some(PathObject::TreeInfo(tree_info)) => {
                        node! { <tbody> {
                            for info in tree_info.entries.iter() {
                                self.show_workspace_file_row(
                                    path_info.workspace_id,
                                    path_info.commit.commit_id.clone(),
                                    path_info.path.clone(),
                                    info,
                                )
                            }
                        } </tbody> }
                    },
                    _ => node! {},
                }
            }
            None => node! {},
        }
    }

    fn show_workspace_file_row(
        &self,
        workspace_id: i64,
        commit_id: String,
        path: String,
        info: &TreeEntryInfo,
    ) -> Node<app::Msg> {
        let info_name = if info.kind == "tree" {
            format!("{}/", info.name)
        } else {
            format!("{}", info.name)
        };
        node! {
            <tr>
                <td class=format!("gitobj-{}", info.kind)><span><a
                    href=format!("file/{}/{}{}",
                        commit_id,
                        path,
                        &info_name,
                    )
                    on_click=move |e| {
                        e.prevent_default();
                        Msg::Retrieve(
                            Resource::WorkspacePathInfo(
                                workspace_id,
                                commit_id.clone(),
                                format!("{}{}", path, &info_name),
                            ),
                            format!("{}/{}{}", commit_id, path, &info_name),
                        )
                    }
                    >{ text!("{}", info.name) }</a></span>
                </td>
                <td></td>
                <td></td>
            </tr>
        }
    }

}
