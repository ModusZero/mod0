use tokio::process::{ChildStdin, ChildStdout};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};
use tokio::sync::Mutex;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};

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

    /// Lee del proceso y emite eventos hacia el frontend
    pub fn spawn_reader(stdout: ChildStdout, event_channel: String, app: AppHandle) {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();

            loop {
                line.clear();
                if reader.read_line(&mut line).await.unwrap_or(0) == 0 { break; }
                
                let trimmed = line.trim();
                if trimmed.is_empty() { continue; }

                // Si detectamos cabecera LSP
                if line.starts_with("Content-Length: ") {
                    if let Ok(len) = line["Content-Length: ".len()..].trim().parse::<usize>() {
                        let mut separator = String::new();
                        let _ = reader.read_line(&mut separator).await;

                        let mut body = vec![0u8; len];
                        if reader.read_exact(&mut body).await.is_ok() {
                            if let Ok(json_val) = serde_json::from_slice::<serde_json::Value>(&body) {
                                let _ = app.emit(&event_channel, json_val);
                            }
                        }
                    }
                } else {
                    // Fallback para logs de herramientas CLI
                    let _ = app.emit(&format!("{}:log", event_channel), trimmed);
                }
            }
        });
    }

    pub async fn send_raw(&self, data: &[u8]) -> Result<(), String> {
        let mut stdin = self.stdin.lock().await;
        stdin.write_all(data).await.map_err(|e| e.to_string())?;
        stdin.flush().await.map_err(|e| e.to_string())
    }
}