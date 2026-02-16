use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Mutex;
use tauri::AppHandle;
use crate::services::bridge::transport::stdio::StdioTransport;
use crate::services::bridge::capabilities::lsp::registry::LspDefinition;
use crate::services::bridge::codec::rpc_json::JsonRpcMessage;

pub struct LspManager {
    pub instances: Arc<Mutex<HashMap<String, StdioTransport>>>,
}

impl LspManager {
    pub fn new() -> Self {
        Self { instances: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub async fn get_or_spawn(&self, lang: &str, app: AppHandle) -> Result<(), String> {
        let mut instances = self.instances.lock().await;
        if instances.contains_key(lang) { return Ok(()); }

        let def = LspDefinition::get_definition(lang)
            .ok_or_else(|| format!("Lenguaje {} no soportado", lang))?;

        let mut child = Command::new(def.command)
            .args(def.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| e.to_string())?;

        let stdin = child.stdin.take().ok_or("STDIN error")?;
        let stdout = child.stdout.take().ok_or("STDOUT error")?;

        StdioTransport::spawn_reader(stdout, format!("lsp-event-{}", lang), app);
        instances.insert(lang.to_string(), StdioTransport::new(stdin));

        Ok(())
    }

    pub async fn send_request(&self, lang: &str, msg: JsonRpcMessage) -> Result<(), String> {
        let mut instances = self.instances.lock().await;
        let transport = instances.get_mut(lang).ok_or("LSP no iniciado")?;
        transport.send(msg).await
    }
}