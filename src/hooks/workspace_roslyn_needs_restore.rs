use async_trait::async_trait;
use lsp_proxy::{Hook, HookOutput, HookResult, Message, Notification, Request, message::Direction};
use serde_json::{Map, Value};

fn get_uuid() -> String {
    let template = "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx";
    let pattern = regex::Regex::new("[xy]").unwrap();
    pattern
        .replace_all(template, |caps: &regex::Captures| {
            let v = if &caps[0] == "x" {
                rand::random_range(0..15)
            } else {
                rand::random_range(8..11)
            };
            format!("{:x}", v)
        })
        .to_string()
}

const RESTORE_REQUEST_ID: i64 = 998;

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
        let mut params = match &notification.params {
            Some(params) => params.clone(),
            None => Value::Object(Map::new()),
        };
        params.as_object_mut().and_then(|p| {
            p.insert(
                "partialResultToken".to_owned(),
                Value::String(self.uuid.clone()),
            )
        });

        Ok(
            HookOutput::new(Message::Notification(notification)).with_messages(vec![(
                Direction::ToServer,
                Message::Request(Request {
                    id: RESTORE_REQUEST_ID,
                    method: "workspace/_roslyn_restore".to_string(),
                    params: Some(params),
                }),
            )]),
        )
    }
}
