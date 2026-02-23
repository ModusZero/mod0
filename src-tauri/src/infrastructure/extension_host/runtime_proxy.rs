use super::manifest::{ExtensionManifest};
use crate::infrastructure::utils::transport::{
    ProtocolType,
    process::ProcessManager,
    stdio::StdioTransport,
    codec::{
        JsonRpcMessage, 
        CodecStrategy, 
        BridgePayload
    }
};
use std::sync::Arc;
use std::path::PathBuf;
use tauri::AppHandle;

pub struct ExtensionRuntime {
    pub id: String,
    pub transport: Arc<StdioTransport>,
    pub strategy: CodecStrategy,
}

impl ExtensionRuntime {
    pub async fn start(manifest: &ExtensionManifest, base_path: PathBuf, app: AppHandle) -> Result<Self, String> {
        // La ruta al binario es absoluta: carpeta_extension + main
        let exe_path = base_path.join(&manifest.main);
        
        let mut child = ProcessManager::spawn_with_pipes(exe_path.to_str().unwrap(), &[])?;
        let stdin = child.stdin.take().ok_or("STDIN_ERROR")?;
        let stdout = child.stdout.take().ok_or("STDOUT_ERROR")?;
        
        let transport = Arc::new(StdioTransport::new(stdin));
        let strategy = match manifest.contributions.protocol {
            ProtocolType::Stdio => CodecStrategy::LspHeader,
            _ => CodecStrategy::NewLine
        };

        // Escucha en background y emite eventos a Tauri para que el Front se entere de logs/errores
        StdioTransport::spawn_reader(stdout, format!("ext:{}", manifest.id), app);

        Ok(Self { id: manifest.id.clone(), transport, strategy })
    }

    pub async fn send_request(&self, method: &str, params: serde_json::Value) -> Result<(), String> {
        let msg = JsonRpcMessage::notification(method, params);
        let payload = BridgePayload::JsonRpc(msg);
        let bytes = payload.encode(self.strategy)?;
        self.transport.send_raw(&bytes).await
    }
}