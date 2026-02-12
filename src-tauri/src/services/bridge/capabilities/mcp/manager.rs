use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::process::Command;
use tokio::io::AsyncBufReadExt;
use tauri::AppHandle;
use crate::services::protocol_engine::transport::ProtocolTransport;
use crate::services::mcp::registry::McpDefinition;

pub struct McpManager {
    pub servers: Arc<Mutex<HashMap<String, ProtocolTransport>>>,
}

impl McpManager {
    pub fn new() -> Self {
        Self {
            servers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn ensure_server(&self, name: &str, app: AppHandle, project_path: &str) -> Result<(), String> {
        let mut servers = self.servers.lock().await;
        
        if servers.contains_key(name) { return Ok(()); }

        let def = McpDefinition::get_core_tools(project_path).into_iter()
            .find(|d| d.name == name)
            .ok_or_else(|| format!("MCP Server {} not found", name))?;

        let mut child = Command::new(&def.command)
            .args(&def.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Fallo al iniciar MCP {}: {}", def.command, e))?;

        let stdin = child.stdin.take().ok_or("No se pudo abrir STDIN del MCP")?;
        let stdout = child.stdout.take().ok_or("No se pudo abrir STDOUT del MCP")?;
        let stderr = child.stderr.take().ok_or("No se pudo abrir STDERR")?;

        let name_for_err = name.to_string(); 
        
        // Monitor de errores (Stderr)
        tokio::spawn(async move {
            let mut reader = tokio::io::BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                eprintln!("[MCP {} Error]: {}", name_for_err, line);
            }
        });

        ProtocolTransport::spawn_reader(stdout, format!("mcp-event-{}", name), app);
        
        servers.insert(name.to_string(), ProtocolTransport::new(stdin));
        
        println!("MCP Server {} iniciado correctamente", name);
        Ok(())
    }
}