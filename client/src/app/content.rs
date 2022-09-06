use crate::app;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

use pmrmodel_base::{
    workspace::{
        JsonWorkspaceRecords,
        WorkspaceRecord,
    },
    git::ObjectInfo,
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
    WorkspaceTop(JsonWorkspaceRecord, Option<ObjectInfo>),
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
            Content::WorkspaceTop(workspace_record, object_info) => {
                node! {
                    <div class="main">
                        <h1>"Workspace"</h1>
                        <div class="workspace-objectinfo">
                            <div>{ text!("{:?}", workspace_record) }</div>
                            <div>{ text!("{:?}", object_info) }</div>
                        </div>
                    </div>
                }
            }
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
}
