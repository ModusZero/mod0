use tokio::sync::RwLock;
use std::sync::Arc;
use crate::infrastructure::clients::defs::{ProviderConfig, AiDriver};
use crate::infrastructure::clients::ai::{
    AiProvider, 
    gemini::GeminiProvider, 
    claude::ClaudeProvider, 
    ollama::OllamaProvider
};

/// Gestor de dominio para la Inteligencia Artificial.
/// Se encarga de la instanciación y el despacho de comandos al driver activo.
pub struct AiManager {
    /// Instancia activa protegida para acceso asíncrono.
    active_driver: RwLock<Option<Arc<dyn AiProvider>>>,
}

impl AiManager {
    pub fn new() -> Self {
        Self { active_driver: RwLock::new(None) }
    }

    /// Determina e instancia el driver correcto basado en la definición técnica.
    pub async fn set_provider(&self, config: ProviderConfig) -> Result<(), String> {
        let driver_type = config.ai_driver.as_ref().ok_or("No AI driver type defined")?;
        
        let driver: Arc<dyn AiProvider> = match driver_type {
            AiDriver::GoogleAI => Arc::new(GeminiProvider::new(config)),
            AiDriver::Anthropic => Arc::new(ClaudeProvider::new(config)),
            AiDriver::Ollama => Arc::new(OllamaProvider::new(config)),
        };

        let mut lock = self.active_driver.write().await;
        *lock = Some(driver);
        Ok(())
    }

    /// Ejecuta la acción de completado sobre el driver actual.
    pub async fn completion(&self, prompt: &str) -> Result<String, String> {
        let lock = self.active_driver.read().await;
        let driver = lock.as_ref().ok_or("AI Provider no inicializado")?;
        driver.completion(prompt).await
    }
}