use sqlx::FromRow;

/// Tabla virtual FTS5 diseñada para búsquedas de texto completo sobre el contenido del proyecto.
#[derive(Debug, FromRow)]
pub struct FileIndex {
    pub path: String,
    pub project_path: String,
    pub content: String,
}