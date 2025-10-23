use anyhow::{Result, bail};
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
        let Some(solution) = solution_path else {
            let Some(projects) = project_paths else {
                bail!("None of the required paths provided");
            };

            return Ok(Notification {
                jsonrpc: String::from("2.0"),
                method: String::from("project/open"),
                params: Params::Project(ProjectParams { projects }),
            });
        };

        return Ok(Notification {
            jsonrpc: String::from("2.0"),
            method: String::from("solution/open"),
            params: Params::Solution(SolutionParams { solution }),
        });
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
        let sln_notification_result = Notification::new(None, None);
        assert!(sln_notification_result.is_err());
    }

    #[test]
    fn notification_new_returns_sln_when_sln_path_is_some() {
        let sln_notification_result =
            Notification::new(Some(String::from("/path/solution.sln")), None);

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
        let sln_notification_result =
            Notification::new(None, Some(vec![String::from("/path/project.csproj")]));

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
        let sln_notification_result = Notification::new(
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
            Notification::new(Some(String::from("/path/solution.sln")), None);

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
