use dashmap::DashMap;
use std::sync::Arc;
use tokio::process::Command;
use std::process::Stdio;
use tauri::AppHandle;
use crate::services::bridge::transport::stdio::StdioTransport;
use crate::services::bridge::defs::{RuntimeDefinition, CapabilityType};

pub struct CapabilityManager {
    /// Sesiones de procesos vivos (LSP/MCP)
    pub active_sessions: Arc<DashMap<String, StdioTransport>>,
}

impl CapabilityManager {
    pub fn new() -> Self {
        Self { active_sessions: Arc::new(DashMap::new()) }
    }

    /// Lanza un proceso basado en una definición enviada por la IA o cargada del config
    pub async fn spawn_runtime(&self, def: RuntimeDefinition, app: AppHandle) -> Result<(), String> {
        if self.active_sessions.contains_key(&def.id) { 
            return Ok(()); 
        }

        let prefix = match def.capability {
            CapabilityType::Lsp => "lsp",
            CapabilityType::Mcp => "mcp",
        };
        
        let event_channel = format!("{}:{}", prefix, def.id);

        let mut child = Command::new(&def.command)
            .args(&def.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Fallo al iniciar {}: {}", def.id, e))?;

        let stdin = child.stdin.take().ok_or("No se pudo capturar STDIN")?;
        let stdout = child.stdout.take().ok_or("No se pudo capturar STDOUT")?;

        StdioTransport::spawn_reader(stdout, event_channel, app);
        
        self.active_sessions.insert(def.id, StdioTransport::new(stdin));

        Ok(())
    }

    pub fn stop_runtime(&self, id: &str) {
        self.active_sessions.remove(id);
    }

    pub fn get_transport(&self, id: &str) -> Option<StdioTransport> {
        self.active_sessions.get(id).map(|s| s.clone())
    }
}