use async_trait::async_trait;
use lsp_proxy::{
    Hook, HookOutput, HookResult, Message,
    hooks::{Direction, Notification, Request},
};

pub struct WorkspaceRoslynNeedsRestore {
    uuid: String,
}

impl WorkspaceRoslynNeedsRestore {
    pub fn new() -> Self {
        let template = "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx";
        let pattern = regex::Regex::new("[xy]").unwrap();
        let uuid = pattern
            .replace_all(template, |caps: &regex::Captures| {
                let r = rand::random::<u8>() % 16;
                let v = if &caps[0] == "x" { r } else { (r & 0x3) | 0x8 };
                format!("{:x}", v)
            })
            .to_string();
        Self { uuid }
    }
}

#[async_trait]
impl Hook for WorkspaceRoslynNeedsRestore {
    async fn on_notification(&self, notification: Notification) -> HookResult {
        Ok(
            HookOutput::new(Message::Notification(notification)).with_messages(vec![(
                Direction::ToServer,
                Message::Request(Request {
                    id: 293,
                    method: "workspace/_roslyn_restore".to_string(),
                    params: Some(serde_json::json!({
                        "partialResultToken": self.uuid
                    })),
                }),
            )]),
        )
    }
}
