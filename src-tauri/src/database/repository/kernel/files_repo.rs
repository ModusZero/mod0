use super::file_metadata::FileMetadata;
use super::Repository;

impl<'a> Repository<'a> {
    /// Indexa el contenido de un archivo en la tabla virtual FTS5.
    /// 
    /// ### Parámetros
    /// * `path`: Ruta del archivo.
    /// * `project_path`: Ruta raíz del proyecto.
    /// * `content`: Texto completo para indexación.
    pub async fn index_file(&self, path: &str, project_path: &str, content: &str) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO file_index (path, project_path, content) VALUES (?, ?, ?)",
            path, project_path, content
        )
        .execute(self.pool).await?;
        Ok(())
    }

    /// Busca términos en el contenido de cada archivo del proyecto devolviendo las rutas más relevantes.
    /// 
    /// ### Parámetros
    /// * `query`: Términos de búsqueda (Sintaxis MATCH de FTS5).
    /// * `project_path`: Ruta del proyecto para delimitar el alcance.
    pub async fn search_code_in_project(&self, query: &str, project_path: &str) -> sqlx::Result<Vec<String>> {
        let results = sqlx::query!(
            r#"SELECT path as "path!: String" 
               FROM file_index 
               WHERE project_path = ? AND content MATCH ? 
               ORDER BY rank"#,
            project_path, query
        )
        .fetch_all(self.pool).await?;
        
        Ok(results.into_iter().map(|r| r.path).collect())
    }

    /// Busca por nombres de archivos en el índice del proyecto devolviendo las rutas más relevantes.
    /// 
    /// ### Parámetros
    /// * `query`: Términos de búsqueda (Sintaxis MATCH de FTS5).
    /// * `project_path`: Ruta del proyecto para delimitar el alcance.
    pub async fn search_filename(&self, query: &str, project_path: &str) -> sqlx::Result<Vec<String>> {
        let pattern = format!("%{}%", query);

        let results = sqlx::query!(
            r#"SELECT path as "path!: String" 
                FROM file_index 
                WHERE project_path = ? AND path LIKE ?"#,
            project_path,
            pattern
        )
        .fetch_all(self.pool).await?;
        
        Ok(results.into_iter().map(|r| r.path).collect())
    }

    /// Elimina un archivo del índice de búsqueda.
    /// 
    /// ### Parámetros
    /// * `path`: Ruta del archivo a eliminar.
    /// * `project_path`: Proyecto al que pertenece el archivo.
    pub async fn delete_from_index(&self, path: &str, project_path: &str) -> sqlx::Result<()> {
        sqlx::query!(
            "DELETE FROM file_index WHERE path = ? AND project_path = ?",
            path, project_path
        )
        .execute(self.pool).await?;
        Ok(())
    }

    /// Actualiza metadatos e índice de contenido en una transacción atómica.
    /// 
    /// ### Parámetros
    /// * `meta`: Estructura con los metadatos del archivo.
    /// * `content`: Contenido para el índice FTS5.
    pub async fn update_file_full(&self, meta: &FileMetadata, content: &str) -> sqlx::Result<()> {
        let mut tx = self.pool.begin().await?;
        
        sqlx::query!(
            "INSERT OR REPLACE INTO file_metadata (path, last_modified, size, file_type, language_id, project_path) 
             VALUES (?, ?, ?, ?, ?, ?)",
            meta.path, meta.last_modified, meta.size, meta.file_type, meta.language_id, meta.project_path
        ).execute(&mut *tx).await?;

        sqlx::query!(
            "INSERT OR REPLACE INTO file_index (path, project_path, content) VALUES (?, ?, ?)",
            meta.path, meta.project_path, content
        ).execute(&mut *tx).await?;

        tx.commit().await
    }
}