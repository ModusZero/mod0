use tokio::process::{ChildStdin, ChildStdout};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};
use tauri::{AppHandle, Emitter};
use crate::services::bridge::codec::rpc_json::JsonRpcMessage;

pub struct StdioTransport {
    stdin: ChildStdin,
}

impl StdioTransport {
    pub fn new(stdin: ChildStdin) -> Self { Self { stdin } }

    /// Lector universal: Puede emitir JSON-RPC o Raw Lines
    pub fn spawn_reader(stdout: ChildStdout, event_channel: String, app: AppHandle) {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();

            loop {
                line.clear();
                if reader.read_line(&mut line).await.is_err() || line.is_empty() { break; }

                // Lógica LSP (Content-Length)
                if line.starts_with("Content-Length: ") {
                    let len: usize = line["Content-Length: ".len()..].trim().parse().unwrap_or(0);
                    let _ = reader.read_line(&mut String::new()).await; // Consumir \r\n

                    let mut body = vec![0u8; len];
                    if reader.read_exact(&mut body).await.is_ok() {
                        if let Ok(msg) = serde_json::from_slice::<JsonRpcMessage>(&body) {
                            let _ = app.emit(&event_channel, msg);
                        }
                    }
                } else {
                    // Fallback a LineStream: Enviar la línea cruda si no es LSP
                    let _ = app.emit(&event_channel, line.trim());
                }
            }
        });
    }

    pub async fn send_raw(&mut self, data: Vec<u8>) -> Result<(), String> {
        self.stdin.write_all(&data).await.map_err(|e| e.to_string())?;
        self.stdin.flush().await.map_err(|e| e.to_string())
    }
}