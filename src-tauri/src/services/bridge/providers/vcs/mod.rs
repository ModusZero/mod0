use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod drivers;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VcsCapabilities {
    pub can_pull_request: bool,
    pub is_cloud: bool,
}

#[async_trait]
pub trait VcsProvider: Send + Sync {
    /// Información sobre qué puede hacer este driver
    fn capabilities(&self) -> VcsCapabilities;

    /// Ejecuta un comando git. 
    /// En lugar de Vec<String>, podrías usar un slice de &str para eficiencia.
    async fn execute_command(&self, args: &[&str], path: &str) -> Result<String, String>;

    /// Crear un Pull Request (opcional, validado por capabilities)
    async fn create_pull_request(&self, title: &str, head: &str, base: &str) -> Result<String, String>;
    
    /// Método de salud para verificar si el token/config es válido antes de usarlo
    async fn check_health(&self) -> bool;
}