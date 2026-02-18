use dashmap::DashMap;
use tokio::sync::RwLock;
use crate::services::bridge::defs::{ProviderConfig, AiDriver, VcsDriver};
use super::ai::{AiProvider};
use super::ai::drivers::{claude::ClaudeProvider, ollama::OllamaProvider, gemini::GeminiProvider};
use super::vcs::{VcsProvider};
use super::vcs::drivers::{github::GithubProvider, local::LocalGitProvider, gitlab::GitlabProvider};

pub struct ProviderManager {
    pub configs: DashMap<String, ProviderConfig>,
    active_ai: RwLock<Option<Box<dyn AiProvider>>>,
    active_vcs: RwLock<Option<Box<dyn VcsProvider>>>,
}

impl ProviderManager {
    pub fn new() -> Self {
        Self {
            configs: DashMap::new(),
            active_ai: RwLock::new(None),
            active_vcs: RwLock::new(None),
        }
    }

    pub async fn init_ai(&self, id: &str) -> Result<(), String> {
        let config = self.configs.get(id).ok_or("Config IA no encontrada")?.clone();
        let driver = config.ai_driver.as_ref().ok_or("Driver IA no definido")?;
        
        let provider: Box<dyn AiProvider> = match driver {
            AiDriver::Anthropic => Box::new(ClaudeProvider::new(config)),
            AiDriver::Ollama => Box::new(OllamaProvider::new(config)),
            AiDriver::GoogleAI => Box::new(GeminiProvider::new(config)),
        };

        *self.active_ai.write().await = Some(provider);
        Ok(())
    }

    pub async fn init_vcs(&self, id: &str) -> Result<(), String> {
        let config = self.configs.get(id).ok_or("Config VCS no encontrada")?.clone();
        let driver = config.vcs_driver.as_ref().ok_or("Driver VCS no definido")?;

        let provider: Box<dyn VcsProvider> = match driver {
            VcsDriver::Github => Box::new(GithubProvider::new(config)),
            VcsDriver::GitLocal => Box::new(LocalGitProvider::new(config)),
            VcsDriver::Gitlab => Box::new(GitlabProvider::new(config)),
            // _ => return Err("Driver VCS no soportado aún".into()),
        };

        *self.active_vcs.write().await = Some(provider);
        Ok(())
    }

    pub async fn get_ai(&self) -> Result<Box<dyn AiProvider>, String> {
        // En Rust, clonar un Box dyn es complejo, lo ideal es usar Arc o devolver referencia
        // Por ahora, para simplificar el copy-paste:
        Err("Usa el método del BridgeService para llamar a la IA".into())
    }

    /// Registra una configuración en el mapa
    pub fn register_config(&self, config: ProviderConfig) {
        self.configs.insert(config.id.clone(), config);
    }

    /// Ejecuta un comando Git usando el driver activo
    pub async fn execute_vcs_command(&self, args: &[&str], path: &str) -> Result<String, String> {
        let vcs_lock = self.active_vcs.read().await;
        let vcs = vcs_lock.as_ref().ok_or("No hay un proveedor VCS activo")?;
        vcs.execute_command(args, path).await
    }

    /// Crea un PR usando el driver activo
    pub async fn create_vcs_pr(&self, title: &str, head: &str, base: &str) -> Result<String, String> {
        let vcs_lock = self.active_vcs.read().await;
        let vcs = vcs_lock.as_ref().ok_or("No hay un proveedor VCS activo")?;
        vcs.create_pull_request(title, head, base).await
    }

    /// Método para que BridgeService pueda usar la IA internamente
    pub async fn completion(&self, prompt: &str) -> Result<String, String> {
        let ai_lock = self.active_ai.read().await;
        let ai = ai_lock.as_ref().ok_or("No hay un proveedor de IA activo")?;
        ai.completion(prompt).await
    }
}