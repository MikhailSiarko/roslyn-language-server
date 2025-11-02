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

pub struct WorkspaceRoslynNeedsRestore;

impl WorkspaceRoslynNeedsRestore {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for WorkspaceRoslynNeedsRestore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Hook for WorkspaceRoslynNeedsRestore {
    async fn on_notification(&self, notification: Notification) -> HookResult {
        let mut params = match notification.params {
            Some(params) => params.clone(),
            None => Value::Object(Map::new()),
        };
        params
            .as_object_mut()
            .and_then(|p| p.insert("partialResultToken".to_owned(), Value::String(get_uuid())));

        Ok(HookOutput::empty().with_messages(vec![(
            Direction::ToServer,
            Message::Request(Request {
                id: rand::random::<i64>(),
                method: "workspace/_roslyn_restore".to_string(),
                params: Some(params),
            }),
        )]))
    }
}
