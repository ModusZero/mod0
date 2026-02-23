use std::fs;
use std::path::PathBuf;
use dashmap::DashMap;
use serde_json::Value;
use crate::domain::external::ai_manager::AiManager;
use crate::domain::external::vcs_manager::VcsManager;
use crate::infrastructure::clients::defs::{ProviderConfig, GitAction};

/// Punto de entrada unificado para todas las capacidades de comunicación (Bridge).
/// Coordina los managers de dominio y la persistencia de configuraciones.
pub struct ExternalService {
    /// Manager de clientes de AI.
    pub ai: AiManager,

    /// Manager de clientes de VCS.
    pub vcs: VcsManager,
    
    /// Almacenamiento en memoria de las configuraciones disponibles.
    pub configs: DashMap<String, ProviderConfig>,

    /// Root path del workspace
    mod0_dir: PathBuf,
}

impl ExternalService {
    pub fn new(project_root: PathBuf) -> Self {
        let mod0_dir = project_root.join(".mod0");
        if !mod0_dir.exists() { let _ = fs::create_dir_all(&mod0_dir); }

        Self {
            ai: AiManager::new(),
            vcs: VcsManager::new(),
            configs: DashMap::new(),
            mod0_dir,
        }
    }

    /// Registra y activa un proveedor por su ID.
    /// Esta función orquesta la carga desde el mapa de configuraciones hacia los managers.
    pub async fn activate_provider(&self, id: &str) -> Result<(), String> {
        let config = self.configs.get(id)
            .ok_or_else(|| format!("Configuración '{}' no encontrada", id))?
            .clone();

        if config.ai_driver.is_some() {
            self.ai.set_provider(config.clone()).await?;
        }
        if config.vcs_driver.is_some() {
            self.vcs.set_provider(config).await?;
        }
        Ok(())
    }

    /// Proxy de alto nivel para consultas de IA.
    pub async fn ask_ai(&self, prompt: &str) -> Result<String, String> {
        self.ai.completion(prompt).await
    }

    /// Proxy de alto nivel para acciones de VCS.
    pub async fn execute_vcs_action(&self, action: &GitAction, args: Value) -> Result<String, String> {
        match action {
            GitAction::Commit => {
                let msg = args["msg"].as_str().ok_or("Falta mensaje de commit")?;
                self.vcs.execute(&["commit", "-m", msg], ".").await
            },
            GitAction::PullRequest => {
                let title = args["title"].as_str().ok_or("Falta título")?;
                let head = args["head"].as_str().ok_or("Falta head")?;
                let base = args["base"].as_str().ok_or("Falta base")?;
                self.vcs.create_pr(title, head, base).await
            },
            _ => Err(format!("Acción VCS no soportada"))
        }
    }

    pub fn register_config(&self, config: ProviderConfig) {
        self.configs.insert(config.id.clone(), config);
    }
}