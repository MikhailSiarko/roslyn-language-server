use async_trait::async_trait;
use lsp_proxy::{
    Hook, HookOutput, HookResult, Message,
    hooks::{Direction, Request},
};
use serde_json::json;

pub struct InitializeHook {
    solution_path: Option<String>,
    projects_path: Vec<String>,
}

impl InitializeHook {
    pub fn new(solution_path: Option<String>, projects_path: Vec<String>) -> Self {
        Self {
            solution_path,
            projects_path,
        }
    }
}

#[async_trait]
impl Hook for InitializeHook {
    async fn on_request(&self, request: Request) -> HookResult {
        let notification = match &self.solution_path {
            Some(sln) => Message::notification("solution/open", Some(json!({ "solution": sln }))),
            None if !self.projects_path.is_empty() => Message::notification(
                "project/open",
                Some(json!({ "projects": self.projects_path })),
            ),
            None => {
                return Ok(HookOutput::new(Message::Request(request)));
            }
        };
        Ok(HookOutput::new(Message::Request(request))
            .with_message(Direction::ToServer, notification))
    }
}
