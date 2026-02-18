use std::sync::Arc;
use tokio::sync::Mutex;
use dashmap::DashMap;
use tauri::{AppHandle};
use std::path::PathBuf;
use crate::kernel::terminal::pty::PtyInstance;

pub struct TerminalManager {
    sessions: Arc<DashMap<String, Arc<Mutex<PtyInstance>>>>,
    app_handle: AppHandle,
}

impl TerminalManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            app_handle,
        }
    }

    pub async fn create_session(&self, id: String, cwd: PathBuf) -> Result<(), String> {
        let instance = PtyInstance::new(id.clone(), cwd, self.app_handle.clone())?;
        self.sessions.insert(id, Arc::new(Mutex::new(instance)));
        Ok(())
    }

    pub async fn write(&self, id: &str, data: &str) -> Result<PathBuf, String> {
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
}