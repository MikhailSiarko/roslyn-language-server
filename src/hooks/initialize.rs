use async_trait::async_trait;
use lsp_proxy::{Hook, HookOutput, HookResult, Message, Request, message::Direction};
use serde_json::json;
use url::Url;

use crate::path;

pub struct InitializeHook {
    solution_path: Option<String>,
    projects_path: Option<Vec<String>>,
}

impl InitializeHook {
    pub fn new(solution_path: Option<String>, projects_path: Option<Vec<String>>) -> Self {
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
            None => match &self.projects_path {
                Some(projs) => {
                    Message::notification("project/open", Some(json!({ "projects": projs })))
                }
                None => {
                    let work_dir = request
                        .params
                        .as_ref()
                        .and_then(|p| p.get("rootUri"))
                        .and_then(|r| r.as_str())
                        .and_then(|r| Url::parse(r).ok())
                        .and_then(|u| u.to_file_path().ok());

                    match work_dir {
                        Some(work_dir) => match path::find_solution_file(work_dir.as_path()) {
                            Some(sln) => Message::notification(
                                "solution/open",
                                Some(json!({ "solution": sln })),
                            ),
                            None => {
                                let projs = path::find_projects_files(work_dir.as_path());
                                Message::notification(
                                    "project/open",
                                    Some(json!({ "projects": projs })),
                                )
                            }
                        },
                        None => panic!("Could not find rootUri property"),
                    }
                }
            },
        };
        Ok(HookOutput::new(Message::Request(request))
            .with_message(Direction::ToServer, notification))
    }
}
