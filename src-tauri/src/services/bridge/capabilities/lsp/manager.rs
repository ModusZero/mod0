use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Mutex;
use tauri::AppHandle;
use crate::services::bridge::codec::rpc_json::ProtocolTransport;
use crate::services::bridge::capabilities::lsp::registry::LspDefinition;

pub struct LspManager {
    pub transports: Arc<Mutex<HashMap<String, ProtocolTransport>>>,
}

impl LspManager {
    pub fn new() -> Self {
        Self {
            transports: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get_or_spawn(&self, lang: &str, app: AppHandle) -> Result<(), String> {
        let mut transports = self.transports.lock().await;
        
        if transports.contains_key(lang) {
            return Ok(());
        }

        let def = LspDefinition::get_definition(lang)
            .ok_or_else(|| format!("Language {} is not supported by Mod0 yet", lang))?;

        let mut child = Command::new(def.command)
            .args(def.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to start LSP server: {}", e))?;

        let stdin = child.stdin.take().ok_or("Failed to open stdin")?;
        let stdout = child.stdout.take().ok_or("Failed to open stdout")?;

        // Canal de eventos único para este lenguaje
        let channel = format!("lsp-event-{}", lang);
        ProtocolTransport::spawn_reader(stdout, channel, app);

        // Almacenamos el transporte genérico
        transports.insert(lang.to_string(), ProtocolTransport::new(stdin));
        
        Ok(())
    }
}