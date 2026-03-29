use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OpenRepositoryStrategy {
    /// Siempre buscar repositorios en carpetas padre.
    Always,
    /// Nunca buscar.
    Never,
    /// Preguntar al usuario.
    Prompt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitConfig {
    pub autofetch: bool,
    pub confirm_sync: bool,
    pub open_repository_in_parent_folders: OpenRepositoryStrategy,
    /// Modelos de IA predeterminados para operaciones GitLens/Copilot equivalentes
    pub ai_model: String,
}