use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

// Centralización del nombre del archivo
pub const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub theme: String,
    pub language: String,
    pub last_project_path: Option<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            language: "es".to_string(),
            last_project_path: None,
        }
    }
}

impl AppConfig {
    pub fn load(config_dir: PathBuf) -> Self {
        let config_path = config_dir.join(CONFIG_FILE_NAME);
        
        if !config_path.exists() {
            fs::create_dir_all(&config_dir).ok();
            let default_config = Self::default();
            let json = serde_json::to_string_pretty(&default_config).unwrap();
            fs::write(config_path, json).ok();
            return default_config;
        }

        let data = fs::read_to_string(config_path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_else(|_| Self::default())
    }

    pub fn save(&self, config_dir: PathBuf) -> Result<(), String> {
        let config_path = config_dir.join(CONFIG_FILE_NAME);
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(config_path, json).map_err(|e| e.to_string())?;
        Ok(())
    }
}