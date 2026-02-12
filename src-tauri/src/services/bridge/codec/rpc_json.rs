use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout};
use tauri::{AppHandle, Emitter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonRpcMessage {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    pub method: Option<String>,
    pub params: Option<serde_json::Value>,
    pub result: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
}

pub struct ProtocolTransport {
    stdin: ChildStdin,
}

impl ProtocolTransport {
    pub fn new(stdin: ChildStdin) -> Self { Self { stdin } }

    pub fn spawn_reader(stdout: ChildStdout, channel: String, app: AppHandle) {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            loop {
                line.clear();
                if reader.read_line(&mut line).await.is_err() || line.is_empty() { break; }
                
                if line.starts_with("Content-Length: ") {
                    let len: usize = line["Content-Length: ".len()..].trim().parse().unwrap_or(0);
                    line.clear();
                    let _ = reader.read_line(&mut line).await; // Consumir el \r\n
                    
                    let mut body = vec![0u8; len];
                    if reader.read_exact(&mut body).await.is_ok() {
                        if let Ok(msg) = serde_json::from_slice::<JsonRpcMessage>(&body) {
                            let _ = app.emit(&channel, msg);
                        }
                    }
                }
            }
        });
    }

    pub async fn send(&mut self, msg: JsonRpcMessage) -> Result<(), String> {
        let json = serde_json::to_string(&msg).map_err(|e| e.to_string())?;
        let payload = format!("Content-Length: {}\r\n\r\n{}", json.len(), json);
        self.stdin.write_all(payload.as_bytes()).await.map_err(|e| e.to_string())?;
        self.stdin.flush().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}