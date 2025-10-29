use std::sync::Arc;

use async_trait::async_trait;
use lsp_proxy::{Hook, HookOutput, HookResult, Message, Notification};
use tokio::sync::Mutex;

use crate::State;

pub struct DocumentDidCloseHook {
    state: Arc<Mutex<State>>,
}

impl DocumentDidCloseHook {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }
}

#[async_trait]
impl Hook for DocumentDidCloseHook {
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
            let mut state = self.state.lock().await;
            if state.opened_file.as_ref() == Some(&uri) {
                state.opened_file = None;
            }
        }

        Ok(HookOutput::new(Message::Notification(notification)))
    }
}
