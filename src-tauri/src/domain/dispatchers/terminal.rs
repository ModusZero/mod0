use std::sync::Arc;
use crate::infrastructure::events;
use crate::infrastructure::terminal::TerminalManager;
use crate::infrastructure::extension_host::ExtensionHost;
use serde_json::json;

pub struct TerminalDispatcher {
    local_manager: Arc<TerminalManager>,
    extension_host: Arc<ExtensionHost>,
}

impl TerminalDispatcher {
    pub async fn execute(&self, session_id: &str, command: &str) -> Result<(), String> {
        // 1. Notificar a las extensiones que un comando va a ejecutarse
        // Esto permite a una extensión de IA explicar el comando si falla
        let _ = self.extension_host.dispatch(
            events::terminal::ON_DATA, 
            json!({ "session_id": session_id, "command": command })
        ).await;

        // 2. Ejecución local (PTY)
        self.local_manager.write_and_log(session_id, command).await?;

        Ok(())
    }
}