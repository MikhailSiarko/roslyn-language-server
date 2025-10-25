use super::LspProxy;
use anyhow::{Context, Result};
use serde_json::Value;
use smol::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::sync::Arc;

/// Header prefix for Content-Length in LSP messages
const CONTENT_LENGTH_HEADER: &str = "Content-Length: ";
/// Header delimiter in LSP messages
const HEADER_DELIMITER: &str = "\r\n\r\n";

/// LSP transport layer that handles communication between client and server
pub struct LspTransport {
    proxy: Arc<LspProxy>,
}

impl LspTransport {
    pub fn new(proxy: LspProxy) -> Self {
        Self {
            proxy: Arc::new(proxy),
        }
    }

    /// Start proxying messages between client and server
    pub async fn start<R, W>(
        &self,
        mut client_reader: R,
        mut client_writer: W,
        mut server_reader: R,
        mut server_writer: W,
    ) where
        R: AsyncRead + Unpin + Send,
        W: AsyncWrite + Unpin + Send,
    {
        // Spawn two tasks: one for client->server and one for server->client
        let proxy = self.proxy.clone();
        let client_to_server = async move {
            loop {
                // Read message from client
                let msg = Self::read_message(&mut client_reader)
                    .await
                    .expect("Failed to read message from client");

                // Process through proxy
                let processed = proxy
                    .process_request(msg)
                    .await
                    .expect("Failed to process client request");

                // Forward to server
                Self::write_message(&mut server_writer, &processed)
                    .await
                    .expect("Failed to write message to server");
            }
        };

        let proxy = self.proxy.clone();
        let server_to_client = async move {
            loop {
                // Read message from server
                let msg = Self::read_message(&mut server_reader)
                    .await
                    .expect("Failed to read message from server");

                // Process through proxy
                let processed = proxy
                    .process_response(msg)
                    .await
                    .expect("Failed to process server response");

                // Forward to client
                Self::write_message(&mut client_writer, &processed)
                    .await
                    .expect("Failed to write message to client");
            }
        };

        // Run both directions concurrently
        smol::future::zip(client_to_server, server_to_client).await;
    }

    /// Read a single LSP message
    async fn read_message<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Value> {
        // Read headers
        let mut header = String::new();
        let mut byte = [0u8; 1];

        loop {
            reader.read_exact(&mut byte).await?;
            header.push(byte[0] as char);

            if header.ends_with(HEADER_DELIMITER) {
                break;
            }
        }

        // Parse content length
        let content_length = header
            .lines()
            .find(|line| line.starts_with(CONTENT_LENGTH_HEADER))
            .and_then(|line| line[CONTENT_LENGTH_HEADER.len()..].parse::<usize>().ok())
            .context("Failed to parse Content-Length header")?;

        // Read message content
        let mut content = vec![0u8; content_length];
        reader.read_exact(&mut content).await?;

        // Parse JSON
        let value = serde_json::from_slice(&content)?;
        Ok(value)
    }

    /// Write a single LSP message
    async fn write_message<W: AsyncWrite + Unpin>(writer: &mut W, msg: &Value) -> Result<()> {
        let content = serde_json::to_vec(msg)?;
        let header = format!(
            "{}Content-Length: {}\r\n\r\n",
            CONTENT_LENGTH_HEADER,
            content.len()
        );

        writer.write_all(header.as_bytes()).await?;
        writer.write_all(&content).await?;
        writer.flush().await?;

        Ok(())
    }
}
