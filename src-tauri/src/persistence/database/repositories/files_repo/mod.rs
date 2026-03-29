use super::file_metadata::FileMetadata;
use super::Repository;

impl<'a> Repository<'a> {
    /// Actualiza metadatos e índice de contenido en una transacción atómica.
    /// 
    /// ### Parámetros
    /// * `meta`: Estructura con los metadatos del archivo.
    /// * `content`: Contenido para el índice FTS5.
    pub async fn update_file_full(&self, meta: &FileMetadata, content: &str) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO file_metadata (path, last_modified, size, file_type, language_id, project_path) 
             VALUES (?, ?, ?, ?, ?, ?)",
            meta.path, meta.last_modified, meta.size, meta.file_type, meta.language_id, meta.project_path
        ).execute(&mut *tx).await?;

        Ok(())
    }
}