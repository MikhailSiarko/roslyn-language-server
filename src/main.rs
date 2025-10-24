use std::path::PathBuf;

use anyhow::{Ok, Result, anyhow};
use clap::Parser;
use roslyn_ls::{args::Args, notification::Notification, path, server};
use smol::{
    Unblock,
    io::{self, AsyncReadExt, AsyncWriteExt, BufReader},
};

fn main() -> Result<()> {
    smol::block_on(async {
        let args = Args::parse();
        let server_path = PathBuf::from(args.cmd);
        let binary = path::get_binary(&server_path)?;
        let logs_path = path::get_logs_path(&server_path).await?;
        let open_notification = match (&args.solution_path, &args.project_paths) {
            (None, None) => Notification::from_working_dir(args.working_dir)?,
            _ => Notification::from_sln_or_proj_path(args.solution_path, args.project_paths)?,
        };

        let (mut server_stdin, server_stdout) = server::start(binary, logs_path).await?;
        let stdin = Unblock::new(std::io::stdin());

        let stream_to_stdout = async {
            let mut reader = BufReader::new(server_stdout);
            let mut stdout = Unblock::new(std::io::stdout());

            io::copy(&mut reader, &mut stdout).await
        };

        let stdin_to_stream = async {
            let mut stdin = BufReader::new(stdin);
            loop {
                let mut buffer = Vec::with_capacity(6000);
                let bytes_read = stdin
                    .read(&mut buffer)
                    .await
                    .expect("Unable to read incoming client notification");

                if bytes_read == 0 {
                    break; // EOF
                }

                server_stdin
                    .write_all(&buffer[..bytes_read])
                    .await
                    .expect("Unable to forward client notification to server");

                let notification = String::from_utf8(buffer[..bytes_read].to_vec())
                    .expect("Unable to convert buffer to string");

                if notification.contains("initialize") {
                    let open_notification_string = open_notification
                        .serialize()
                        .expect("Unable to serialize open solution/project notification");

                    server_stdin
                        .write_all(open_notification_string.as_bytes())
                        .await
                        .expect("Unable to send open solution notification to server");

                    break;
                }
            }
            io::copy(&mut stdin, &mut server_stdin).await
        };

        let (stdin_result, stdout_result) =
            smol::future::zip(stdin_to_stream, stream_to_stdout).await;

        match (stdin_result, stdout_result) {
            (Err(stdin_err), Err(stdout_err)) => Err(anyhow!(
                "Both stdin and stdout streams encountered errors\n\t- {}\n\t- {}",
                stdin_err,
                stdout_err
            )),
            (Err(stdin_err), _) => Err(anyhow!(stdin_err)),
            (_, Err(stdout_err)) => Err(anyhow!(stdout_err)),
            _ => Ok(()),
        }
    })
}
