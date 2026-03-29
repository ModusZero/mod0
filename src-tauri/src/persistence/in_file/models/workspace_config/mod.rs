use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum WorkspaceTrustLevel {
    /// Workspace no confiable, funciones de ejecución desactivadas.
    Restricted,
    /// Workspace confiable, ejecución total permitida.
    Trusted,
}

/// Espacio de trabajo indexado, almacenando su ubicación física y preferencias globales.v
#[derive(Debug, FromRow)]
pub struct Workspace {
    pub path: String,
    pub name: String,
    pub last_opened: DateTime<Utc>,
    pub trust_level: WorkspaceTrustLevel,
    /// Tags para organización (persistido como JSON string).
    pub tags: String, 
    // Estos dos siguientes deben ser listas
    // Aqui se guardaran las instaladas especificamente en este workspace y las activadas (instaladas globalmente) solo en este workspace
    // pub installed_extensions: ExtensionRecord,
    // pub active_global_extensions: ExtensionRecord
}