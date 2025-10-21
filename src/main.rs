use std::path::{Path, PathBuf};

use anyhow::{Ok, Result};
use clap::Parser;
use roslyn_ls::{args::Args, path, server};
use smol::{
    Unblock,
    io::{self, AsyncReadExt, AsyncWriteExt, BufReader},
};

fn main() -> Result<()> {
    smol::block_on(async {
        let args = Args::parse();
        println!("{:#?}", args);
        let binary = path::get_binary(Path::new(&args.cmd))?;
        let logs_path = PathBuf::from(args.logs_path);
        let (mut server_stdin, server_stdout) = server::start(binary, &logs_path).await?;

        let stdin = Unblock::new(std::io::stdin());

        let stream_to_stdout = async {
            let mut reader = BufReader::new(server_stdout);
            let mut stdout = Unblock::new(std::io::stdout());

            io::copy(&mut reader, &mut stdout).await
        };

        let stdin_to_stream = async {
            let mut stdin = BufReader::new(stdin);
            loop {
                let mut buffer = vec![0; 6000];
                let bytes_read = stdin
                    .read(&mut buffer)
                    .await
                    .expect("Unable to read incoming client notification");
                if bytes_read == 0 {
                    break; // EOF reached
                }
                server_stdin
                    .write_all(&buffer[..bytes_read])
                    .await
                    .expect("Unable to forward client notification to server");

                let notification = String::from_utf8(buffer[..bytes_read].to_vec())
                    .expect("Unable to convert buffer to string");

                if notification.contains("initialize") {
                    let open_solution_notification = create_open_notification(
                        &notification,
                        args.solution_path,
                        args.project_paths,
                    );

                    server_stdin
                        .write_all(open_solution_notification.as_bytes())
                        .await
                        .expect("Unable to send open solution notification to server");

                    break;
                }
            }
            io::copy(&mut stdin, &mut server_stdin).await
        };

        smol::future::zip(stdin_to_stream, stream_to_stdout);
        Ok(())
    })
}
