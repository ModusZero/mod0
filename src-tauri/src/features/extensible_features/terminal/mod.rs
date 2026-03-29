pub mod default;
pub mod internal_logic;

use std::sync::Arc;
use tokio::sync::Mutex;
use dashmap::DashMap;
use tauri::{AppHandle};
use std::path::PathBuf;
use default::pty_terminal::PtyInstance;
use internal_logic::persistence::TerminalPersistence;
use crate::persistence::in_file::{DbManager, models::kernel::terminal_history::TerminalHistory};

pub mod terminal_events {
    pub const ON_DATA: &str = "term.onData";
    pub const ON_COMMAND_EXECUTED: &str = "term.onCommandExecuted";
}

// TODO implementarlo de forma que para cada accion busque si hay extensiones activas que contribuyan a ellas.
// De haber varias, debe buscar la de mayor prioridad, y delegarle la tarea, pero sincronizando de
// igual los `shared behaviors`

// Algo como:

// use std::sync::Arc;
// use crate::infrastructure::events;
// use crate::infrastructure::terminal::TerminalManager;
// use crate::infrastructure::extensions::ExtensionHost;
// use serde_json::json;

// pub struct TerminalDispatcher {
//     local_manager: Arc<TerminalManager>,
//     extension_host: Arc<ExtensionHost>,
// }

// impl TerminalDispatcher {
//     pub async fn execute(&self, session_id: &str, command: &str) -> Result<(), String> {
//         // 1. Notificar a las extensiones que un comando va a ejecutarse
//         // Esto permite a una extensión de IA explicar el comando si falla
//         let _ = self.extension_host.dispatch(
//             events::terminal::ON_DATA, 
//             json!({ "session_id": session_id, "command": command })
//         ).await;

//         // 2. Ejecución local (PTY)
//         self.local_manager.write_and_log(session_id, command).await?;

//         Ok(())
//     }
// }

pub struct TerminalManager {
    sessions: Arc<DashMap<String, Arc<Mutex<PtyInstance>>>>,
    app_handle: AppHandle,
    db: DbManager
}

impl TerminalManager {
    pub fn new(app_handle: AppHandle, db: DbManager) -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            app_handle,
            db
        }
    }

    pub async fn create_session(&self, id: String, cwd: PathBuf) -> Result<(), String> {
        let instance = PtyInstance::new(id.clone(), cwd, self.app_handle.clone())?;
        self.sessions.insert(id, Arc::new(Mutex::new(instance)));
        Ok(())
    }

    pub async fn write_and_log(&self, id: &str, data: &str) -> Result<(), String> {
        let cwd = self.write(id, data).await?;

        let db_clone = self.db.clone();
        let id_clone = id.to_string();
        let data_clone = data.to_string();
        let cwd_str = cwd.to_string_lossy().to_string();

        tokio::spawn(async move {
            TerminalPersistence::log_command(
                db_clone,
                id_clone,
                cwd_str,
                data_clone
            ).await;
        });

        Ok(())
    }

    async fn write(&self, id: &str, data: &str) -> Result<PathBuf, String> {
        if let Some(instance_arc) = self.sessions.get(id) {
            let mut instance = instance_arc.lock().await;
            instance.write(data)?;
            Ok(instance.cwd.clone()) // Retornamos el CWD para que el command sepa dónde loguear
        } else {
            Err("Sesión no encontrada".into())
        }
    }

    pub async fn resize(&self, id: &str, rows: u16, cols: u16) -> Result<(), String> {
        if let Some(instance_arc) = self.sessions.get(id) {
            instance_arc.lock().await.resize(rows, cols)
        } else {
            Err("Sesión no encontrada".into())
        }
    }

    pub async fn get_capture(&self, id: &str) -> Result<String, String> {
        if let Some(instance_arc) = self.sessions.get(id) {
            Ok(instance_arc.lock().await.read_buffer())
        } else {
            Err("Sesión no encontrada".into())
        }
    }

    pub async fn get_history(&self, session_id: &str) -> Result<Vec<TerminalHistory>, String> {
        TerminalPersistence::get_history(self.db.clone(), session_id.to_string()).await
    }

    pub async fn clear_history(&self, session_id: &str) -> Result<(), String> {
        TerminalPersistence::clear_history(self.db.clone(), session_id.to_string()).await
    }
}