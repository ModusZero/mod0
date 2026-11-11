use std::path::PathBuf;
use std::sync::Arc;

use tauri::{AppHandle, Manager};

use crate::config::ConfigStore;
use crate::core::error::AppResult;
use crate::features::sessions::manager::SessionManager;

/// Rutas persistentes de la aplicación.
#[derive(Debug, Clone)]
pub struct Paths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
}

impl Paths {
    pub fn resolve(app: &AppHandle) -> AppResult<Self> {
        let config_dir = app.path().app_config_dir()?;
        let data_dir = app.path().app_data_dir()?;
        std::fs::create_dir_all(&config_dir)?;
        std::fs::create_dir_all(&data_dir)?;
        Ok(Self { config_dir, data_dir })
    }

    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.json")
    }

    pub fn sessions_file(&self) -> PathBuf {
        self.data_dir.join("sessions.json")
    }

    pub fn graph_file(&self, session_id: &str) -> PathBuf {
        self.data_dir.join("graphs").join(format!("{session_id}.json"))
    }

    pub fn hooks_file(&self, session_id: &str) -> PathBuf {
        self.data_dir.join("hooks").join(format!("{session_id}.json"))
    }

    pub fn board_file(&self, project_key: &str) -> PathBuf {
        self.data_dir.join("boards").join(format!("{project_key}.json"))
    }
}

/// Estado global inyectado en todos los comandos de Tauri.
pub struct AppState {
    pub paths: Paths,
    pub config: Arc<ConfigStore>,
    pub sessions: Arc<SessionManager>,
}
