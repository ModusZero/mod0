use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Inventario global de capacidades instaladas, gestionando versiones y esquemas de configuración de las extensiones.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Extension {
    pub id: String,
    pub version: String,
    pub local_path: String,
    pub is_enabled: bool,
    pub config_schema: Option<String>,
    pub manifest_cache: Option<String>,
}