use async_trait::async_trait;
use lsp_proxy::hooks::Notification;
use lsp_proxy::{Hook, HookOutput, HookResult, Message};

pub struct DocumentDidOpenHook;

impl DocumentDidOpenHook {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DocumentDidOpenHook {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Hook for DocumentDidOpenHook {
    async fn on_notification(&self, notification: Notification) -> HookResult {
        // let mut state = proxy.state().lock().unwrap();
        // let uri = match &notification.params {
        //     Some(params) => params
        //         .get("textDocument")
        //         .and_then(|td| td.get("uri"))
        //         .and_then(|u| u.as_str())
        //         .map(|s| s.to_string()),
        //     None => None,
        // };

        // if let Some(uri) = uri {
        //     state.opened_file.replace(uri);
        // }

        Ok(HookOutput::new(Message::Notification(notification)))
    }
}
