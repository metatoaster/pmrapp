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

use crate::app::Fetch;
use crate::app::Msg;

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Clone, derive_more::From,
)]
pub enum Content {
    WorkspaceListing(JsonWorkspaceRecords),
    Workspace(ObjectInfo),
}


impl Content {
    pub fn view(&self) -> Node<app::Msg> {
        match self {
            Content::WorkspaceListing(records) => {
                node! {
                    <div class="workspace-listing">
                    {
                        for record in &records.workspaces {
                            self.show_workspace_record_row(record)
                        }
                    }
                    </div>
                }
            },
            Content::Workspace(record) => {
                node! {
                    <div class="workspace-objectinfo">
                        <div>{ text!("{:?}", record) }</div>
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
                        Msg::Fetching(Fetch::Workspace(workspace_id), format!("/workspace/{}/", workspace_id))
                    }
                >{ text!("Workspace: {}", workspace_id) }
                </a></div>
                <div>{ text!("{}", workspace_record.url) }</div>
                <div>{ text!("{}", workspace_record.description.as_ref().unwrap_or(&"".to_string())) }</div>
            </div>
        }
    }
}
