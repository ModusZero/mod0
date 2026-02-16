use dashmap::DashMap;
use std::sync::Arc;
use tokio::process::Command;
use std::process::Stdio;
use tauri::{AppHandle, Manager};
use crate::services::bridge::transport::stdio::StdioTransport;
use crate::services::bridge::registry::defs::RuntimeDefinition;

pub struct CapabilityManager {
    active_sessions: Arc<DashMap<String, StdioTransport>>,
}

impl CapabilityManager {
    pub fn new() -> Self {
        Self { active_sessions: Arc::new(DashMap::new()) }
    }

    pub async fn spawn_runtime(&self, def: RuntimeDefinition, app: AppHandle) -> Result<(), String> {
        if self.active_sessions.contains_key(&def.id) { return Ok(()); }

        let project_path = app.path().app_local_data_dir().unwrap_or_default();

        let mut child = Command::new(&def.command)
            .args(&def.args)
            .current_dir(project_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Error ejecutando {}: {}", def.name, e))?;

        let stdin = child.stdin.take().ok_or("Error: STDIN no disponible")?;
        let stdout = child.stdout.take().ok_or("Error: STDOUT no disponible")?;

        let event_channel = format!("runtime:{}", def.id);
        
        // Spawn del lector que enviará mensajes al canal 'runtime:id'
        StdioTransport::spawn_reader(stdout, event_channel, app);
        
        // Guardamos el transporte para enviar mensajes desde el backend
        self.active_sessions.insert(def.id, StdioTransport::new(stdin));

        Ok(())
    }

    pub fn stop_runtime(&self, id: &str) {
        self.active_sessions.remove(id);
    }
}