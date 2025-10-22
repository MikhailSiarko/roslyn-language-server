use anyhow::{Result, anyhow, bail};
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Params {
    Solution(SolutionParams),
    Project(ProjectParams),
}

#[derive(Serialize, Debug)]
pub struct Notification {
    pub jsonrpc: String,
    pub method: String,
    pub params: Params,
}

#[derive(Serialize, Debug)]
pub struct SolutionParams {
    pub solution: String,
}

#[derive(Serialize, Debug)]
pub struct ProjectParams {
    pub projects: Vec<String>,
}

impl Notification {
    pub fn new(solution_path: Option<String>, project_paths: Option<Vec<String>>) -> Result<Self> {
        if let (None, None) = (&solution_path, &project_paths) {
            bail!("None of the required paths provided");
        }

        if let Some(solution) = solution_path {
            return Ok(Notification {
                jsonrpc: String::from("2.0"),
                method: String::from("solution/open"),
                params: Params::Solution(SolutionParams { solution }),
            });
        }

        if let Some(projects) = project_paths {
            return Ok(Notification {
                jsonrpc: String::from("2.0"),
                method: String::from("project/open"),
                params: Params::Project(ProjectParams { projects }),
            });
        }

        Err(anyhow!("None of the required paths provided"))
    }

    pub fn serialize(self) -> Result<String> {
        let body = serde_json::to_string(&self)?;
        Ok(generate_message(&body))
    }
}

fn generate_message(body: &str) -> String {
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    let full_message = format!("{header}{body}");

    full_message
}
