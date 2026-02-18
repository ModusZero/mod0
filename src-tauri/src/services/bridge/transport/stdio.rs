use tokio::process::{ChildStdin, ChildStdout};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};
use tokio::sync::Mutex;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use crate::services::bridge::codec::rpc_json::JsonRpcMessage;

#[derive(Clone)]
pub struct StdioTransport {
    stdin: Arc<Mutex<ChildStdin>>,
}

impl StdioTransport {
    pub fn new(stdin: ChildStdin) -> Self {
        Self {
            stdin: Arc::new(Mutex::new(stdin)),
        }
    }

    pub fn spawn_reader(stdout: ChildStdout, event_channel: String, app: AppHandle) {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) | Err(_) => break, // Stream cerrado o error
                    Ok(_) => {
                        let trimmed = line.trim();
                        if trimmed.is_empty() { continue; }

                        // Lógica LSP (Content-Length)
                        if line.starts_with("Content-Length: ") {
                            if let Ok(len) = line["Content-Length: ".len()..].trim().parse::<usize>() {
                                // Consumir la línea vacía (\r\n) que separa headers de body
                                let mut separator = String::new();
                                let _ = reader.read_line(&mut separator).await;

                                let mut body = vec![0u8; len];
                                if reader.read_exact(&mut body).await.is_ok() {
                                    if let Ok(msg) = serde_json::from_slice::<JsonRpcMessage>(&body) {
                                        let _ = app.emit(&event_channel, msg);
                                    }
                                }
                            }
                        } else {
                            // Fallback: Emitir como texto plano (útil para logs o CLI tools)
                            let _ = app.emit(&event_channel, trimmed);
                        }
                    }
                }
            }
        });
    }

    pub async fn send_raw(&self, data: Vec<u8>) -> Result<(), String> {
        let mut stdin = self.stdin.lock().await;
        stdin.write_all(&data).await.map_err(|e| e.to_string())?;
        stdin.flush().await.map_err(|e| e.to_string())
    }
}