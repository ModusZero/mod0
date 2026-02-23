use async_trait::async_trait;
use crate::infrastructure::dispatchers::defs::ProviderConfig;

/// Contrato fundamental para proveedores de Inteligencia Artificial.
/// Define la estructura que cualquier driver debe implementar.
#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Crea una nueva instancia del driver con su configuración.
    fn new(config: ProviderConfig) -> Self where Self: Sized;
    /// Ejecuta una solicitud de completado de texto.
    async fn completion(&self, prompt: &str) -> Result<String, String>;
}

pub mod gemini;
pub mod claude;
pub mod ollama;