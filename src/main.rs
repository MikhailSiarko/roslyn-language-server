use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;
use clap::Parser;
use roslyn_ls::{
    args::Args,
    hooks::InitializeHook,
    path::{self},
};
use smol::{
    Unblock,
    io::{BufReader, BufWriter},
};

fn main() -> Result<()> {
    smol::block_on(async {
        let args = Args::parse();
        let server_path = PathBuf::from(args.cmd);
        let cmd = path::cmd(&server_path)?;
        let workspace_path = Path::new(&args.working_dir);
        let solution_path = path::find_solution_file(workspace_path);
        let projects_path = path::find_projects_files(workspace_path);
        let logs_path = path::get_logs_path(&server_path).await?;
        let stdin = Unblock::new(std::io::stdin());
        let stdout = Unblock::new(std::io::stdout());

        let proxy = lsp_proxy::ProxyBuilder::new()
            .with_hook(
                "initialize",
                Arc::new(InitializeHook::new(solution_path, projects_path)),
            )
            .build();

        let logs_arg = format!("--extensionLogDirectory={}", logs_path.display());
        proxy
            .spawn(
                &cmd,
                &["--logLevel=Information", logs_arg.as_str(), "--stdio"],
                BufReader::new(stdin),
                BufWriter::new(stdout),
            )
            .await?;
        Ok(())
    })
}
