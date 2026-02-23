pub mod pty;
pub mod persistence;

use std::sync::Arc;
use tokio::sync::Mutex;
use dashmap::DashMap;
use tauri::{AppHandle};
use std::path::PathBuf;
use crate::database::{DbManager, models::kernel::terminal_history::TerminalHistory};
use self::pty::PtyInstance;
use self::persistence::TerminalPersistence;

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct TerminalOutput {
    pub session_id: String,
    pub data: String,
}

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