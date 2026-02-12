use super::Repository;

impl<'a> Repository<'a> {
    pub async fn index_file(&self, path: &str, content: &str) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO file_index (path, content) VALUES (?, ?)",
            path, content
        )
        .execute(self.pool).await?;
        Ok(())
    }

    pub async fn search_code(&self, query: &str) -> sqlx::Result<Vec<String>> {
        let results = sqlx::query!(
            r#"SELECT path as "path: String" FROM file_index WHERE content MATCH ? ORDER BY rank LIMIT 50"#,
            query
        )
        .fetch_all(self.pool)
        .await?;

        // .flatten() convierte Option<String> en String, descartando los None
        Ok(results.into_iter().map(|r| r.path).flatten().collect())
    }

    pub async fn delete_from_index(&self, path: &str) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM file_index WHERE path = ?", path)
            .execute(self.pool).await?;
        Ok(())
    }
}