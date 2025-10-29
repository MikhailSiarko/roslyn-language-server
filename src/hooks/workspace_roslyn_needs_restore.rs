use async_trait::async_trait;
use lsp_proxy::{
    Hook, HookOutput, HookResult, Message,
    hooks::{Direction, Notification, Request},
};

fn get_uuid() -> String {
    let template = "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx";
    let pattern = regex::Regex::new("[xy]").unwrap();
    pattern
        .replace_all(template, |caps: &regex::Captures| {
            let r = rand::random::<u8>() % 16;
            let v = if &caps[0] == "x" { r } else { (r & 0x3) | 0x8 };
            format!("{:x}", v)
        })
        .to_string()
}

const RESTORE_REQUEST_ID: i64 = 293;

pub struct WorkspaceRoslynNeedsRestore {
    uuid: String,
}

impl WorkspaceRoslynNeedsRestore {
    pub fn new() -> Self {
        Self { uuid: get_uuid() }
    }
}

#[async_trait]
impl Hook for WorkspaceRoslynNeedsRestore {
    async fn on_notification(&self, notification: Notification) -> HookResult {
        Ok(
            HookOutput::new(Message::Notification(notification)).with_messages(vec![(
                Direction::ToServer,
                Message::Request(Request {
                    id: RESTORE_REQUEST_ID,
                    method: "workspace/_roslyn_restore".to_string(),
                    params: Some(serde_json::json!({
                        "partialResultToken": self.uuid
                    })),
                }),
            )]),
        )
    }
}
