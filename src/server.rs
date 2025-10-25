use anyhow::{Result, anyhow};
use smol::process::Command;
use std::path::PathBuf;
use std::process::Stdio;

use crate::path::Binary;

pub async fn start<'a>(
    binary: Binary<'a>,
    logs_dir: PathBuf,
) -> Result<(smol::process::ChildStdin, smol::process::ChildStdout)> {
    let mut command = match binary {
        Binary::Exe(path) => Command::new(path),
        Binary::Dll(path) => {
            let mut cmd = Command::new("dotnet");
            cmd.arg("exec");
            cmd.arg(path);
            cmd
        }
    };

    let logs_dir_str = logs_dir.display().to_string();
    let log_arg = format!("--extensionLogDirectory={}", logs_dir_str);
    let command = command
        .arg("--logLevel=Information")
        .arg(log_arg)
        .arg("--stdio")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    match (command.stdin, command.stdout) {
        (Some(stdin), Some(stdout)) => Ok((stdin, stdout)),
        _ => Err(anyhow!("Failed to start language server process")),
    }
}
