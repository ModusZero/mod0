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

/// Almacena atributos físicos y lógicos de los archivos, permitiendo al sistema identificar lenguajes de programación y cambios recientes.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FileMetadata {
    pub path: String,
    pub size: i64,
    pub last_modified: DateTime<Utc>,
    pub file_type: Option<FileType>,
    pub project_path: String,
}