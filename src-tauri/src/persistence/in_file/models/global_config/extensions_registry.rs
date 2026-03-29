use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// TODO Cambiar esto para que sea una lista de extensiones, dicha lista tiene que poder filtrarse por:
// - INSTALADAS globalmente (siempre se guardan en el rootPath de la app)
// - ACTIVADAS globalmente (El registro se guarda en un toml en el rootPath de la app y cada workspace tiene acceso a ellas inicialmente. 
// - INSTALADAS desde un workspace (se guardan en el rootPath del workspace, se borran si se elimina el workspace contenedor, no se pueden desactivar, se supone que si la tienes instalada en un workspace y la quieres desactivar, es mejor desinstalarla)
// - ACTIVADAS en un workspace (Las instaladas globalmente pero no activadas globalmente, para casos frecuentes pero no absolutos)
// Tambien debe permitir simular la funcionalidad de VSCode de recomendar extensiones para un workspace, debe generar como un package.json de dependencias pero con los mcp, lsp, extensiones, rules y skills recomendados asociadas al workspace, que cada workspace sea como un docker container, sin importar el stack

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

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum ExtensionStatus {
    Installed,
    Updating,
    Error,
    Disabled,
}

#[derive(Debug, FromRow)]
pub struct ExtensionRecord {
    pub id: String,
    pub version: String,
    pub publisher: String,
    pub entry_point: String,
    pub status: ExtensionStatus,
    /// Manifest completo en BSON para acceso instantáneo.
    pub manifest_blob: Vec<u8>,
}