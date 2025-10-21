use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Command to start Microsoft.CodeAnalysis.LanguageServer
    #[arg(short, long)]
    pub cmd: String,

    /// Absolute paths to a logs directory
    #[arg(short, long)]
    pub logs_path: String,

    /// Absolute path to a solution (.sln) file
    #[arg(short, long)]
    pub solution_path: Option<String>,

    /// Absolute paths to project(s) (.csproj) files. Ignored if correct solution path is provided
    #[arg(short, long)]
    pub project_paths: Option<Vec<String>>,

    /// Settings for the language server
    #[arg(long)]
    pub settings: Option<Settings>,
}

#[derive(Parser, Debug, Clone, Copy, Deserialize)]
pub struct Settings {
    pub inlay_hints_enabled: Option<bool>,
    pub code_lens_enabled: Option<bool>,
    pub analyzers_enabled: Option<bool>,
}

impl std::str::FromStr for Settings {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
}
