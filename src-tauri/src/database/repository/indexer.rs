use super::Repository;

impl<'a> Repository<'a> {
    /// Inserta o actualiza el contenido de un archivo en la tabla virtual FTS5 para búsqueda de texto.
    pub async fn index_file(&self, path: &str, content: &str) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO file_index (path, content) VALUES (?, ?)",
            path, content
        )
        .execute(self.pool).await?;
        Ok(())
    }

    /// Busca términos en el contenido indexado y devuelve las rutas de los archivos más relevantes.
    pub async fn search_code(&self, query: &str) -> sqlx::Result<Vec<String>> {
        let results = sqlx::query!(
            r#"SELECT path as "path: String" FROM file_index WHERE content MATCH ? ORDER BY rank LIMIT 50"#,
            query
        )
        .fetch_all(self.pool)
        .await?;

        Ok(results.into_iter().filter_map(|r| r.path).collect())
    }

    /// Elimina un archivo del índice de búsqueda mediante su ruta.
    pub async fn delete_from_index(&self, path: &str) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM file_index WHERE path = ?", path)
            .execute(self.pool).await?;
        Ok(())
    }
}