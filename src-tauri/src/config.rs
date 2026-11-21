use std::path::PathBuf;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

use crate::core::error::AppResult;
use crate::core::state::Paths;
use crate::features::notifications::NotificationConfig;
use crate::features::sessions::profile::{CliProfile, default_profiles};
use crate::infrastructure::storage;

/// Número máximo de proyectos recientes que se recuerdan.
const MAX_RECENT_PROJECTS: usize = 12;

/// Configuración global de MOD0 (persistida en `config.json`).
///
/// Todos los campos tienen valor por defecto para que una configuración antigua
/// o parcial siga cargando después de actualizar la app.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AppConfig {
    pub theme: String,
    pub accent: String,
    pub language: String,
    pub ui_scale: f64,
    pub reduce_motion: bool,
    /// Guarda el archivo del editor automáticamente poco después de dejar de escribir.
    pub auto_save: bool,
    pub terminal_font_size: u8,
    pub context_limit: u64,
    pub shell: Option<String>,
    pub last_project_path: Option<String>,
    pub recent_projects: Vec<String>,
    pub profiles: Vec<CliProfile>,
    pub default_profile_id: String,
    pub notifications: NotificationConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".into(),
            accent: "blue".into(),
            language: "es".into(),
            ui_scale: 1.0,
            reduce_motion: false,
            auto_save: false,
            terminal_font_size: 13,
            context_limit: 200_000,
            shell: None,
            last_project_path: None,
            recent_projects: Vec::new(),
            profiles: default_profiles(),
            default_profile_id: "claude".into(),
            notifications: NotificationConfig::default(),
        }
    }
}

/// Acceso concurrente y persistencia de [`AppConfig`].
pub struct ConfigStore {
    path: PathBuf,
    inner: RwLock<AppConfig>,
}

impl ConfigStore {
    pub fn load(paths: &Paths) -> Self {
        let path = paths.config_file();
        let mut config: AppConfig = storage::read_json(&path).ok().flatten().unwrap_or_default();

        if config.profiles.is_empty() {
            config.profiles = default_profiles();
        }

        Self {
            path,
            inner: RwLock::new(config),
        }
    }

    pub fn get(&self) -> AppConfig {
        self.inner.read().expect("config lock").clone()
    }

    pub fn replace(&self, config: AppConfig) -> AppResult<AppConfig> {
        *self.inner.write().expect("config lock") = config.clone();
        storage::write_json_atomic(&self.path, &config)?;
        Ok(config)
    }

    /// Registra un proyecto como el último usado y lo sube al inicio de los recientes.
    pub fn touch_project(&self, project: &str) -> AppResult<AppConfig> {
        let mut config = self.get();
        config.last_project_path = Some(project.to_string());
        config.recent_projects.retain(|p| p != project);
        config.recent_projects.insert(0, project.to_string());
        config.recent_projects.truncate(MAX_RECENT_PROJECTS);
        self.replace(config)
    }
}
