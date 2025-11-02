use std::sync::Arc;

use async_trait::async_trait;
use lsp_proxy::{Hook, HookOutput, HookResult, Message, Notification, Request, message::Direction};
use tokio::sync::Mutex;

use crate::State;

pub struct DocumentDidOpenHook {
    state: Arc<Mutex<State>>,
}

impl DocumentDidOpenHook {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
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

        if let Some(uri) = &uri {
            let mut state = self.state.lock().await;
            state.opened_file.replace(uri.clone());
            return Ok(
                HookOutput::new(Message::Notification(notification)).with_message(
                    Direction::ToServer,
                    Message::Request(Request {
                        id: rand::random::<i64>(),
                        method: "textDocument/diagnostic".to_string(),
                        params: Some(serde_json::json!({
                            "textDocument": {
                                "uri": uri
                            }
                        })),
                    }),
                ),
            );
        }

        Ok(HookOutput::new(Message::Notification(notification)))
    }
}
