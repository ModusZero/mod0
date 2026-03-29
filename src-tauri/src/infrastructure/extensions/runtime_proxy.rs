use super::manifest::ExtensionManifest;
use crate::infrastructure::utils::transport::{
    ProtocolType,
    process::ProcessManager,
    stdio::StdioTransport,
    codec::CodecStrategy
};
use std::sync::Arc;
use std::path::PathBuf;
use tauri::AppHandle;
use serde_json::{Value, json};
use uuid::Uuid;

pub struct ExtensionRuntime {
    pub id: String,
    pub transport: Arc<StdioTransport>,
    pub strategy: CodecStrategy,
}

impl ExtensionRuntime {
    pub async fn start(manifest: &ExtensionManifest, base_path: PathBuf, app: AppHandle) -> Result<Self, String> {
        let exe_path = base_path.join(&manifest.main);
        
        let mut child = ProcessManager::spawn_with_pipes(exe_path.to_str().unwrap(), &[])?;
        let stdin = child.stdin.take().ok_or("STDIN_ERROR")?;
        let stdout = child.stdout.take().ok_or("STDOUT_ERROR")?;
        
        let transport = Arc::new(StdioTransport::new(stdin));
        let strategy = match manifest.contributions.protocol {
            ProtocolType::Stdio => CodecStrategy::LspHeader,
            _ => CodecStrategy::NewLine
        };

        // Iniciamos el lector de fondo vinculando el transporte
        transport.spawn_reader(stdout, format!("ext:{}", manifest.id), app);

        Ok(Self { id: manifest.id.clone(), transport, strategy })
    }

    /// Envía un comando y espera la respuesta del JSON-RPC
    pub async fn send_request_with_response(&self, method: &str, params: Value) -> Result<Value, String> {
        let id = Uuid::new_v4().to_string();
        
        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        let payload = match self.strategy {
            CodecStrategy::LspHeader => {
                let body = serde_json::to_string(&request).map_err(|e| e.to_string())?;
                format!("Content-Length: {}\r\n\r\n{}", body.len(), body).into_bytes()
            }
            CodecStrategy::NewLine => {
                let mut body = serde_json::to_vec(&request).map_err(|e| e.to_string())?;
                body.push(b'\n');
                body
            }
            CodecStrategy::Raw => {
                
            }
        };

        // Usamos el transporte para enviar y esperar el oneshot
        let response = self.transport.send_and_wait(id, payload).await?;
        
        // Extraer el campo "result" o "error" del estándar JSON-RPC
        if let Some(error) = response.get("error") {
            return Err(error.to_string());
        }

        Ok(response.get("result").unwrap_or(&Value::Null).clone())
    }
}