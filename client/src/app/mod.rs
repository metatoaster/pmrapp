use pmrmodel_base::{
    workspace::{
        JsonWorkspaceRecords,
        // WorkspaceRecord,
    },
    git::ObjectInfo,
};
use sauron::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen_futures::spawn_local;
use web_sys::PopStateEvent;

mod content;

use content::Content;
use crate::error::ServerError;
use crate::api;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FetchStatus<T> {
    Idle,
    Complete(T),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Fetch {
    WorkspaceListing,
    Workspace(i64),
}

pub enum Msg {
    Fetching(Fetch),

    // new content and url
    ReceivedContent(Content),
    // for dealing with error responses
    RequestError(ServerError),
    // for the URL push state
    UrlChanged(Fetch, String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct App {
    pub content: FetchStatus<Content>,
    is_loading: bool,
    fetch: Option<Fetch>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            content: FetchStatus::Idle,
            is_loading: true,
            fetch: None,
        }
    }
}

trait Resource<T> {
    fn to_url(&self) -> String;
    fn from_url(url: &str) -> Option<T>;
}

struct WorkspaceListing {
}

impl Resource<()> for WorkspaceListing {
    // TODO figure out how to apply uritemplates into this mess
    fn to_url(&self) -> String {
        format!("/workspace/")
    }

    fn from_url(url: &str) -> Option<()> {
        match url {
            "/workspace/" => Some(()),
            _ => None,
        }
    }
}

struct WorkspaceItem {
    id: i64,
}

impl Resource<i64> for WorkspaceItem {
    // TODO figure out how to apply uritemplates into this mess
    fn to_url(&self) -> String {
        format!("/workspace/{}/", &self.id)
    }

    fn from_url(url: &str) -> Option<i64> {
        if url.starts_with("/workspace/") {
            let parts = url.split("/").collect::<Vec<_>>();
            if parts.len() >= 3 {
                parts[2].parse::<i64>().ok()
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}

impl Application<Msg> for App {

    #[cfg(feature = "wasm")]
    fn init(&mut self) -> Cmd<Self, Msg> {
        // Only calling this init if this is not default, i.e. not the default app
        // created when loading failed in client/src/lib.rs main function
        if self.fetch.is_none() {
            return Cmd::none();
        }
        let mut commands = vec![];
        let listen_to_url_changes = Window::add_event_listeners(vec![on_popstate(|e| {
            log::trace!("pop_state is triggered in sauron add event listener - state: {:#?}", PopStateEvent::from(JsValue::from(&e)).state());
            let url = sauron::window()
                .location()
                .pathname()
                .expect("must have get a pathname");
            // TODO if the state is unsupported, this blows up
            Msg::UrlChanged(
                PopStateEvent::from(JsValue::from(e)).state().into_serde::<Fetch>().unwrap(),
                url
            )
        })]);

        let history = sauron::window().history().expect("must have history");
        log::trace!("setting initial state: {:#?}", self.fetch);
        history
            .replace_state(&JsValue::from_serde(&Some(self.fetch.as_ref())).unwrap(), "")
            .expect("must push state");

        commands.push(listen_to_url_changes);

        /*
        match self.content {
            FetchStatus::Idle => {
                // TODO figure out what are the default things to fetch??
                commands.push(self.fetch_workspace_listing())
            }
            _ => (),
        }
        */
        Cmd::batch(commands)
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <body class="main">
                <header>
                    <a relative href="/workspace/"
                        on_click=|e| {
                            e.prevent_default();
                            Msg::Fetching(Fetch::WorkspaceListing)
                        }>
                        "Workspace Listing"
                    </a>
                </header>
                <main class="content">
                    { self.view_content() }
                </main>
            </body>
        }
    }

    #[cfg(not(feature = "wasm"))]
    fn update(&mut self, _msg: Msg) -> Cmd<Self, Msg> {
        Cmd::none()
    }

    #[cfg(feature = "wasm")]
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            // core application related
            Msg::Fetching(fetch) => {
                match fetch {
                    Fetch::WorkspaceListing => {
                        Self::push_state(fetch, "/workspace/");
                        self.is_loading = true;
                        log::trace!("pushed Msg::Fetching(Fetch::WorkspaceListing)");
                        self.fetch_workspace_listing()
                    }
                    Fetch::Workspace(workspace_id) => {
                        Self::push_state(fetch, &(WorkspaceItem { id: workspace_id }).to_url());
                        self.is_loading = true;
                        log::trace!("pushed Msg::Fetching(Fetch::Workspace(id))");
                        self.fetch_workspace(workspace_id)
                    }
                }
            }

            // System related
            Msg::ReceivedContent(content) => {
                self.content = FetchStatus::Complete(content);
                self.is_loading = false;
                Window::scroll_to_top()
            }
            Msg::RequestError(server_error) => {
                self.is_loading = false;
                log::error!("Error: {}", server_error);
                Cmd::none()
            }
            Msg::UrlChanged(fetch, url) => {
                self.is_loading = true;
                match fetch {
                    Fetch::WorkspaceListing => {
                        self.fetch_workspace_listing()
                    },
                    Fetch::Workspace(workspace_id) => {
                        self.fetch_workspace(workspace_id)
                    },
                    // _ => {
                    //     Cmd::none()
                    // },
                }
            }
        }
    }
}

impl App {
    fn view_content(&self) -> Node<Msg> {
        match &self.content {
            FetchStatus::Idle => node! { <p>"idling"</p> },
            FetchStatus::Error(e) => {
                node! {
                    <article>
                        <p>"Error: "</p>
                        <code>{text(e)}</code>
                    </article>
                }
            }
            FetchStatus::Complete(content) => content.view(),
        }
    }
}

impl App {
    pub fn with_workspace_listing(workspace_listing: JsonWorkspaceRecords) -> Self {
        Self {
            content: FetchStatus::Complete(Content::from(workspace_listing)),
            is_loading: false,
            fetch: Some(Fetch::WorkspaceListing),
        }
    }

    pub fn with_workspace(workspace_id: i64, object_info: ObjectInfo) -> Self {
        Self {
            content: FetchStatus::Complete(Content::from(object_info)),
            is_loading: false,
            fetch: Some(Fetch::Workspace(workspace_id)),
        }
    }
}

#[cfg(feature = "wasm")]
impl App {
    fn fetch_workspace_listing(&self) -> Cmd<Self, Msg> {
        Cmd::new(move|program| {
            let async_fetch = |program:Program<Self,Msg>| async move{
                match api::get_workspace_listing().await {
                    Ok(workspace_records) => {
                        program.dispatch(Msg::ReceivedContent( Content::from(
                            workspace_records,
                        )));
                    }
                    Err(e) => {
                        program.dispatch(Msg::RequestError(e));
                    }
                }
            };
           spawn_local(async_fetch(program))
        })
    }

    fn fetch_workspace(&self, workspace_id: i64) -> Cmd<Self, Msg> {
        Cmd::new(move|program| {
            let async_fetch = |program:Program<Self,Msg>| async move{
                match api::get_workspace(workspace_id).await {
                    Ok(object_info) => {
                        program.dispatch(Msg::ReceivedContent( Content::from(
                            object_info,
                        )));
                    }
                    Err(e) => {
                        program.dispatch(Msg::RequestError(e));
                    }
                }
            };
           spawn_local(async_fetch(program))
        })
    }

    fn push_state(fetch: Fetch, url: &str) {
        let history = sauron::window().history().expect("must have history");
        log::trace!("pushing to state: {}", url);
        history
            .push_state_with_url(&JsValue::from_serde(&fetch).unwrap(), "", Some(url))
            .expect("must push state");
    }
}
