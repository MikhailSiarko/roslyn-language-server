use std::path::Path;

use anyhow::{Result, bail};
use serde::Serialize;

use crate::path;

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
    pub fn from_sln_or_proj_path(
        solution_path: Option<String>,
        project_paths: Option<Vec<String>>,
    ) -> Result<Self> {
        let Some(solution) = solution_path else {
            let Some(projects) = project_paths else {
                bail!("None of the required paths provided")
            };

            return Ok(Notification {
                jsonrpc: String::from("2.0"),
                method: String::from("project/open"),
                params: Params::Project(ProjectParams { projects }),
            });
        };

        Ok(Notification {
            jsonrpc: String::from("2.0"),
            method: String::from("solution/open"),
            params: Params::Solution(SolutionParams { solution }),
        })
    }

    pub fn from_working_dir(working_dir: Option<String>) -> Result<Self> {
        let Some(working_dir) = working_dir else {
            bail!("No working directory provided")
        };

        let working_path = Path::new(&working_dir);
        let Some(solution) = path::find_solution_file(working_path) else {
            let projects = path::find_projects_files(working_path);
            if projects.is_empty() {
                bail!("No solution or project found")
            };

            return Ok(Notification {
                jsonrpc: String::from("2.0"),
                method: String::from("project/open"),
                params: Params::Project(ProjectParams { projects }),
            });
        };

        Ok(Notification {
            jsonrpc: String::from("2.0"),
            method: String::from("solution/open"),
            params: Params::Solution(SolutionParams { solution }),
        })
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

#[cfg(test)]
mod test {
    use crate::notification::{Notification, Params};

    #[test]
    fn notification_new_returns_error_when_no_paths_provided() {
        let sln_notification_result = Notification::from_sln_or_proj_path(None, None);
        assert!(sln_notification_result.is_err());
    }

    #[test]
    fn notification_new_returns_sln_when_sln_path_is_some() {
        let sln_notification_result =
            Notification::from_sln_or_proj_path(Some(String::from("/path/solution.sln")), None);

        match sln_notification_result {
            Ok(Notification {
                jsonrpc: _,
                method: _,
                params: Params::Solution(solution_params),
            }) if solution_params.solution == "/path/solution.sln" => (),
            _ => panic!(""),
        }
    }

    #[test]
    fn notification_new_returns_proj_when_proj_path_is_some() {
        let sln_notification_result = Notification::from_sln_or_proj_path(
            None,
            Some(vec![String::from("/path/project.csproj")]),
        );

        match sln_notification_result {
            Ok(Notification {
                jsonrpc: _,
                method: _,
                params: Params::Project(project_params),
            }) if project_params
                .projects
                .contains(&String::from("/path/project.csproj")) =>
            {
                ()
            }
            _ => panic!(""),
        }
    }

    #[test]
    fn notification_new_returns_sln_when_sln_and_proj_path_is_some() {
        let sln_notification_result = Notification::from_sln_or_proj_path(
            Some(String::from("/path/solution.sln")),
            Some(vec![String::from("/path/project.csproj")]),
        );

        match sln_notification_result {
            Ok(Notification {
                jsonrpc: _,
                method: _,
                params: Params::Solution(solution_params),
            }) if solution_params.solution == "/path/solution.sln" => (),
            _ => panic!(""),
        }
    }

    #[test]
    fn notification_serialize_returns_valid_str_when_sln_path_is_some() {
        let sln_notification_result =
            Notification::from_sln_or_proj_path(Some(String::from("/path/solution.sln")), None);

        match sln_notification_result {
            Ok(notification) => {
                let result = notification.serialize();
                let content = "{\"jsonrpc\":\"2.0\",\"method\":\"solution/open\",\"params\":{\"solution\":\"/path/solution.sln\"}}";
                match result {
                    Ok(json)
                        if json.eq(&format!(
                            "Content-Length: {}\r\n\r\n{}",
                            content.len(),
                            content
                        )) =>
                    {
                        ()
                    }
                    _ => panic!(""),
                }
            }
            _ => panic!(""),
        }
    }
}
