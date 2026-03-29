use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UntrustedFilesStrategy {
    /// Abre archivos no confiables en modo de solo lectura estricto.
    Restricted,
    /// Abre archivos no confiables normalmente (peligroso).
    Open,
    /// Pregunta siempre antes de abrir.
    Prompt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityConfig {
    pub workspace_trust_untrusted_files: UntrustedFilesStrategy,
    /// Rutas de repositorios donde los agentes tienen permiso de ejecución total.
    pub trusted_agent_workspaces: Vec<String>,
}