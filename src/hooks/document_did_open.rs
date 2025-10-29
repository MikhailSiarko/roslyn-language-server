use std::sync::Arc;

use async_trait::async_trait;
use lsp_proxy::hooks::Notification;
use lsp_proxy::{Hook, HookOutput, HookResult, Message};
use tokio::sync::RwLock;

use crate::State;

pub struct DocumentDidOpenHook {
    state: Arc<RwLock<State>>,
}

impl DocumentDidOpenHook {
    pub fn new(state: Arc<RwLock<State>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl Hook for DocumentDidOpenHook {
    async fn on_notification(&self, notification: Notification) -> HookResult {
        let uri = match &notification.params {
            Some(params) => params
                .get("textDocument")
                .and_then(|td| td.get("uri"))
                .and_then(|u| u.as_str())
                .map(|s| s.to_string()),
            None => None,
        };

        if let Some(uri) = uri {
            let mut state = self.state.write().await;
            state.opened_file.replace(uri);
        }

        Ok(HookOutput::new(Message::Notification(notification)))
    }
}
