use async_trait::async_trait;
use crate::infrastructure::dispatchers::defs::{ProviderConfig, VcsCapabilities};

/// Contrato fundamental para sistemas de control de versiones.
#[async_trait]
pub trait VcsProvider: Send + Sync {
    fn new(config: ProviderConfig) -> Self where Self: Sized;
    
    /// Información sobre qué puede hacer este driver
    fn capabilities(&self) -> VcsCapabilities;

    /// Método de salud para verificar si el token/config es válido antes de usarlo
    async fn check_health(&self) -> bool;

    /// Ejecuta un comando git.
    async fn execute(&self, args: &[&str], path: &str) -> Result<String, String>;

    /// Crear un Pull Request (opcional, validado por capabilities)
    async fn create_pr(&self, title: &str, head: &str, base: &str) -> Result<String, String>;
}

pub mod github;
pub mod gitlab;
pub mod local;