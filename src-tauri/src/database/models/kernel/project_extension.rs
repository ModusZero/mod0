use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Tabla de asociación que define qué extensiones están activas en un proyecto específico y sus valores de configuración particulares.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProjectExtension {
    pub project_path: String,
    pub extension_id: String,
    pub config_values: Option<String>,
}