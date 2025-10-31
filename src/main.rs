use std::{io::Error, path::PathBuf, process::Stdio, sync::Arc};

use anyhow::Result;
use clap::Parser;
use roslyn_ls::{
    State,
    args::Args,
    hooks::{
        DocumentDidCloseHook, DocumentDidOpenHook, InitializeHook, RemoveParams,
        WorkspaceProjectInitializationComplete, WorkspaceRoslynNeedsRestore,
    },
    path::{self},
};
use tokio::{
    io::{BufReader, BufWriter, stdin, stdout},
    process::Command,
    sync::Mutex,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let server_path = PathBuf::from(args.cmd);
    let logs_path = path::get_logs_path(&server_path)
        .await?
        .display()
        .to_string();
    let stdin = stdin();
    let stdout = stdout();

    let mut lsp = Command::new("dotnet")
        .args(vec![
            "exec".to_owned(),
            server_path.display().to_string(),
            "--logLevel=Information".to_owned(),
            format!("--extensionLogDirectory={logs_path}"),
            "--stdio".to_owned(),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()?;

    let server_writer = lsp
        .stdin
        .take()
        .map(BufWriter::new)
        .ok_or(Error::other("Failed to get stdin"))?;
    let server_reader = lsp
        .stdout
        .take()
        .map(BufReader::new)
        .ok_or(Error::other("Failed to get stdout"))?;

    let state = Arc::new(Mutex::new(State { opened_file: None }));
    let proxy = lsp_proxy::ProxyBuilder::new()
        .with_hook(
            "initialize",
            Arc::new(InitializeHook::new(args.solution_path, args.project_paths)),
        )
        .with_hook(
            "workspace/projectInitializationComplete",
            Arc::new(WorkspaceProjectInitializationComplete::new(state.clone())),
        )
        .with_hook(
            "workspace/_roslyn_projectNeedsRestore",
            Arc::new(WorkspaceRoslynNeedsRestore::new()),
        )
        .with_hook(
            "textDocument/didOpen",
            Arc::new(DocumentDidOpenHook::new(state.clone())),
        )
        .with_hook(
            "textDocument/didClose",
            Arc::new(DocumentDidCloseHook::new(state.clone())),
        )
        .with_hook(
            "workspace/diagnostic/refresh",
            Arc::new(RemoveParams::new()),
        )
        .with_hook("workspace/inlayHint/refresh", Arc::new(RemoveParams::new()))
        .with_hook("workspace/codeLens/refresh", Arc::new(RemoveParams::new()))
        .build();

    proxy
        .forward(server_reader, server_writer, stdin, stdout)
        .await?;

    drop(lsp);
    Ok(())
}
