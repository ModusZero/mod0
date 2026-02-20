use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Categorización binaria de archivos para determinar estrategias de lectura y procesamiento por parte de la IA.
/// - `Text`: Archivo de texto visible y editable por el `Text Editor`.
/// - `Binary`: Archivo visible por previewers específicos, no editables.
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum FileType {
    Text,
    Binary,
}

/// Espacio de trabajo indexado, almacenando su ubicación física y preferencias globales.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub path: String,
    pub last_opened: DateTime<Utc>,
    pub is_favorite: bool,
    pub custom_settings: Option<String>,
}

/// Tabla virtual FTS5 diseñada para búsquedas de texto completo sobre el contenido del proyecto.
#[derive(Debug, FromRow)]
pub struct FileIndex {
    pub path: String,
    pub project_path: String,
    pub content: String,
}

/// Almacena atributos físicos y lógicos de los archivos, permitiendo al sistema identificar lenguajes de programación y cambios recientes.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FileMetadata {
    pub path: String,
    pub last_modified: DateTime<Utc>,
    pub size: i64,
    pub file_type: Option<FileType>,
    pub language_id: Option<String>,
    pub project_path: String,
}

/// Inventario global de capacidades instaladas, gestionando versiones y esquemas de configuración de las extensiones.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ExtensionRegistry {
    pub id: String,
    pub version: String,
    pub local_path: String,
    pub is_enabled: bool,
    pub config_schema: Option<String>,
    pub manifest_cache: Option<String>,
}

/// Tabla de asociación que define qué extensiones están activas en un proyecto específico y sus valores de configuración particulares.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProjectExtension {
    pub project_path: String,
    pub extension_id: String,
    pub config_values: Option<String>,
}