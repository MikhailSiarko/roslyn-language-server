use async_trait::async_trait;
use lsp_proxy::{Hook, HookOutput, HookResult, Message, hooks::Notification};

pub struct WorkspaceProjectInitializationComplete;

impl WorkspaceProjectInitializationComplete {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WorkspaceProjectInitializationComplete {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Hook for WorkspaceProjectInitializationComplete {
    async fn on_notification(&self, notification: Notification) -> HookResult {
        Ok(HookOutput::new(Message::Notification(notification)))
        // let mut state = proxy.state().lock().unwrap();
        // let opened_file = match state.opened_file {
        //     Some(ref uri) => uri.clone(),
        //     None => return Ok(HookOutput::new(Message::Notification(notification))),
        // };

        // let id = match state.latest_request_id.clone().map(|i| i.as_u64().unwrap()) {
        //     Some(i) => i + 1,
        //     None => 1,
        // };

        // state.latest_request_id = Some(serde_json::Value::from(id));

        // Ok(
        //     HookOutput::new(Message::Notification(notification)).with_messages(vec![
        //         Message::Request(Request {
        //             id: serde_json::Value::from(id),
        //             method: "textDocument/diagnostic".to_string(),
        //             params: Some(serde_json::json!({
        //                 "textDocument": {
        //                     "uri": opened_file
        //                 }
        //             })),
        //         }),
        //     ]),
        // )
    }
}
