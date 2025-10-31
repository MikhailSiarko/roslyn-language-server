use std::sync::Arc;

use async_trait::async_trait;
use lsp_proxy::{Hook, HookOutput, HookResult, Message, Notification, Request, message::Direction};
use tokio::sync::Mutex;

use crate::State;

const DIAGNOSTIC_REQUEST_ID: i64 = 932;

pub struct WorkspaceProjectInitializationComplete {
    state: Arc<Mutex<State>>,
}

impl WorkspaceProjectInitializationComplete {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl Hook for WorkspaceProjectInitializationComplete {
    async fn on_notification(&self, _: Notification) -> HookResult {
        let opened_file = match self.state.lock().await.opened_file {
            Some(ref uri) => uri.clone(),
            None => return Ok(HookOutput::empty()),
        };

        Ok(HookOutput::empty().with_messages(vec![(
            Direction::ToServer,
            Message::Request(Request {
                id: DIAGNOSTIC_REQUEST_ID,
                method: "textDocument/diagnostic".to_string(),
                params: Some(serde_json::json!({
                    "textDocument": {
                        "uri": opened_file
                    }
                })),
            }),
        )]))
    }
}
