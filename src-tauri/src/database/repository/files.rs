use sqlx::{QueryBuilder, Sqlite};
use crate::database::models::files::{FileIndex, FileMetadata, Skill};

use super::Repository;

impl<'a> Repository<'a> {
    /// Inserta o actualiza el contenido de un archivo en la tabla virtual FTS5 para búsqueda de texto.
    pub async fn index_file(&self, file: &FileIndex) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO file_index (path, content) VALUES (?, ?)",
            file.path, 
            file.content
        )
        .execute(self.pool)
        .await?;
        
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

    /// Ejecuta Batched Inserts para cambios masivos en el workspace
    pub async fn update_file_full(&self, meta: &FileMetadata, content: &str) -> sqlx::Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // 1. Guardar metadatos (para FS y explorador)
        sqlx::query!(
            "INSERT OR REPLACE INTO file_metadata (path, last_modified, size, file_type, language_id, project_path) 
             VALUES (?, ?, ?, ?, ?, ?)",
            meta.path, meta.last_modified, meta.size, meta.file_type, meta.language_id, meta.project_path
        ).execute(&mut *tx).await?;

        // 2. Actualizar índice de búsqueda (para el Omnibar/Search)
        sqlx::query!(
            "INSERT OR REPLACE INTO file_index (path, content) VALUES (?, ?)",
            meta.path, content
        ).execute(&mut *tx).await?;

        tx.commit().await
    }

    pub async fn upsert_skill(&self, skill: &Skill) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO skill (id, name, code, description, tags) 
               VALUES (?, ?, ?, ?, ?)
               ON CONFLICT(id) DO UPDATE SET 
               code = excluded.code, 
               description = excluded.description, 
               tags = excluded.tags"#,
            skill.id, skill.name, skill.code, skill.description, skill.tags
        )
        .execute(self.pool).await?;
        Ok(())
    }

    /// Busca habilidades que contengan al menos una de las etiquetas proporcionadas (Filtro OR).
    pub async fn get_skills_by_tags(&self, tags: Vec<String>) -> sqlx::Result<Vec<Skill>> {
        if tags.is_empty() {
            return Ok(vec![]);
        }

        // Usamos QueryBuilder porque la cantidad de tags es dinámica y query! no lo soporta.
        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "SELECT id, name, code, description, tags FROM skill WHERE "
        );

        for (i, tag) in tags.iter().enumerate() {
            if i > 0 {
                query_builder.push(" OR ");
            }
            query_builder.push("tags LIKE ");
            // SQLx gestiona el binding de forma segura para evitar inyecciones SQL.
            query_builder.push_bind(format!("%{}%", tag));
        }

        query_builder
            .build_query_as::<Skill>()
            .fetch_all(self.pool)
            .await
    }
}