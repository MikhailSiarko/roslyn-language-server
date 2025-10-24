use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Command to start Microsoft.CodeAnalysis.LanguageServer
    #[arg(short, long)]
    pub cmd: String,

    /// Absolute path to a working directory
    #[arg(short, long)]
    pub working_dir: Option<String>,

    /// Absolute path to a solution (.sln) file
    #[arg(short, long)]
    pub solution_path: Option<String>,

    /// Absolute paths to project(s) (.csproj) files. Ignored if correct solution path is provided
    #[arg(short, long)]
    pub project_paths: Option<Vec<String>>,
}
