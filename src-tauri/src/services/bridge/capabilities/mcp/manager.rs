use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Mutex;
use tauri::AppHandle;
use crate::services::bridge::transport::stdio::StdioTransport;
use crate::services::bridge::capabilities::mcp::registry::McpDefinition;

pub struct McpManager {
    pub servers: Arc<Mutex<HashMap<String, StdioTransport>>>,
}

impl McpManager {
    pub fn new() -> Self {
        Self { servers: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub async fn ensure_server(&self, name: &str, project_path: &str, app: AppHandle) -> Result<(), String> {
        let mut servers = self.servers.lock().await;
        if servers.contains_key(name) { return Ok(()); }

        // Discovery dinámico
        let def = McpDefinition::discover(name, project_path)
            .ok_or_else(|| format!("MCP {} no encontrado", name))?;

        let mut child = Command::new(&def.command)
            .args(&def.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| e.to_string())?;

        let stdin = child.stdin.take().ok_or("STDIN MCP error")?;
        let stdout = child.stdout.take().ok_or("STDOUT MCP error")?;

        StdioTransport::spawn_reader(stdout, format!("mcp-event-{}", name), app);
        servers.insert(name.to_string(), StdioTransport::new(stdin));

        Ok(())
    }
}