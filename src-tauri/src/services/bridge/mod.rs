pub mod defs;
pub mod codec;
pub mod transport;
pub mod capabilities;
pub mod providers;

use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use serde_json::Value;
use crate::services::bridge::defs::{RuntimeDefinition, ProviderConfig};

pub struct BridgeService {
    pub capabilities: capabilities::manager::CapabilityManager,
    pub providers: providers::manager::ProviderManager,
    mod0_dir: PathBuf,
}

impl BridgeService {
    pub fn new(project_root: PathBuf) -> Self {
        let mod0_dir = project_root.join(".mod0");
        if !mod0_dir.exists() { let _ = fs::create_dir_all(&mod0_dir).ok(); }

        Self {
            capabilities: capabilities::manager::CapabilityManager::new(),
            providers: providers::manager::ProviderManager::new(),
            mod0_dir,
        }
    }

    // --- Lógica de Runtimes (Capabilities) ---

    pub async fn spawn_runtime(&self, def: RuntimeDefinition, app: AppHandle) -> Result<(), String> {
        self.capabilities.spawn_runtime(def, app).await
    }

    pub fn stop_runtime(&self, id: &str) -> Result<(), String> {
        self.capabilities.stop_runtime(id);
        Ok(())
    }

    pub fn load_runtime_configs(&self) -> Vec<RuntimeDefinition> {
        let path = self.mod0_dir.join("capabilities.json");
        fs::read_to_string(path)
            .and_then(|data| serde_json::from_str(&data).map_err(|e| e.into()))
            .unwrap_or_default()
    }

    // --- Lógica de Providers (AI / VCS) ---

    pub fn register_provider(&self, config: ProviderConfig) -> Result<(), String> {
        self.providers.register_config(config);
        Ok(())
    }

    pub async fn activate_provider(&self, id: &str) -> Result<(), String> {
        let config = self.providers.configs.get(id)
            .ok_or_else(|| format!("Configuración '{}' no encontrada", id))?
            .clone();

        if config.ai_driver.is_some() {
            self.providers.init_ai(id).await?;
        }
        if config.vcs_driver.is_some() {
            self.providers.init_vcs(id).await?;
        }
        Ok(())
    }

    pub async fn ask_ai(&self, prompt: &str) -> Result<String, String> {
        self.providers.completion(prompt).await
    }

    pub async fn execute_vcs_action(&self, action: &str, args: Value) -> Result<String, String> {
        match action {
            "commit" => {
                let msg = args["msg"].as_str().ok_or("Falta mensaje de commit")?;
                self.providers.execute_vcs_command(&["commit", "-m", msg], ".").await
            },
            "pr" => {
                let title = args["title"].as_str().ok_or("Falta título")?;
                let head = args["head"].as_str().ok_or("Falta head")?;
                let base = args["base"].as_str().ok_or("Falta base")?;
                self.providers.create_vcs_pr(title, head, base).await
            },
            _ => Err(format!("Acción VCS '{}' no soportada", action))
        }
    }
}