use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Command to start Microsoft.CodeAnalysis.LanguageServer
    #[arg(short, long)]
    pub cmd: String,

    /// Absolute path to a working directory
    #[arg(short, long)]
    pub working_dir: String,
}
