use std::sync::Arc;

use async_trait::async_trait;
use lsp_proxy::{
    Hook, HookOutput, HookResult, Message,
    hooks::{Direction, Notification, Request},
};
use tokio::sync::RwLock;

use crate::State;

pub struct WorkspaceProjectInitializationComplete {
    state: Arc<RwLock<State>>,
}

impl WorkspaceProjectInitializationComplete {
    pub fn new(state: Arc<RwLock<State>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl Hook for WorkspaceProjectInitializationComplete {
    async fn on_notification(&self, notification: Notification) -> HookResult {
        let opened_file = match self.state.read().await.opened_file {
            Some(ref uri) => uri.clone(),
            None => return Ok(HookOutput::new(Message::Notification(notification))),
        };

        Ok(
            HookOutput::new(Message::Notification(notification)).with_messages(vec![(
                Direction::ToServer,
                Message::Request(Request {
                    id: 555,
                    method: "textDocument/diagnostic".to_string(),
                    params: Some(serde_json::json!({
                        "textDocument": {
                            "uri": opened_file
                        }
                    })),
                }),
            )]),
        )
    }
}
