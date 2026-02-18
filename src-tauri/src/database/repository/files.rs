use chrono::Utc;
use sqlx::{QueryBuilder, Sqlite};
use crate::database::models::files::{FileMetadata, Skill};
use super::Repository;

impl<'a> Repository<'a> {
    /// Inserta o actualiza el contenido de un archivo en la tabla virtual FTS5 para búsqueda de texto.
    pub async fn index_file(&self, path: &str, project_path: &str, content: &str) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT OR REPLACE INTO file_index (path, project_path, content) VALUES (?, ?, ?)",
            path, project_path, content
        )
        .execute(self.pool).await?;
        Ok(())
    }

    /// Busca términos en el contenido indexado del workspace y devuelve las rutas de los archivos más relevantes.
    pub async fn search_code_in_project(&self, query: &str, project_path: &str) -> sqlx::Result<Vec<String>> {
        let results = sqlx::query!(
            r#"SELECT path as "path: String" 
               FROM file_index 
               WHERE project_path = ? AND content MATCH ? 
               ORDER BY rank LIMIT 50"#,
            project_path, query
        )
        .fetch_all(self.pool).await?;
        
        Ok(results.into_iter().filter_map(|r| r.path).collect())
    }

    /// Elimina un archivo del índice de búsqueda mediante su ruta y pertenencia al proyecto.
    pub async fn delete_from_index(&self, path: &str, project_path: &str) -> sqlx::Result<()> {
        sqlx::query!(
            "DELETE FROM file_index WHERE path = ? AND project_path = ?",
            path, project_path
        )
        .execute(self.pool).await?;
        Ok(())
    }

    /// Ejecuta una actualización atómica de metadatos e índice de búsqueda en una sola transacción.
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

    /// Registra o actualiza una habilidad, gestionando el conflicto por ID para actualizar sus campos.
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

    /// Recupera habilidades que contengan al menos una de las etiquetas proporcionadas mediante un filtro OR.
    pub async fn get_skills_by_tags(&self, tags: Vec<String>) -> sqlx::Result<Vec<Skill>> {
        if tags.is_empty() {
            return Ok(vec![]);
        }

        let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
            "SELECT id, name, code, description, tags FROM skill WHERE "
        );

        for (i, tag) in tags.iter().enumerate() {
            if i > 0 {
                query_builder.push(" OR ");
            }
            query_builder.push("tags LIKE ");
            query_builder.push_bind(format!("%{}%", tag));
        }

        query_builder
            .build_query_as::<Skill>()
            .fetch_all(self.pool)
            .await
    }

    /// Actualiza el contador de uso y la marca temporal de la última vez que se utilizó una habilidad.
    pub async fn track_skill_usage(&self, skill_id: i64) -> sqlx::Result<()> {
        let now = Utc::now(); 
        sqlx::query!(
            "UPDATE skill SET usage_count = usage_count + 1, last_used = ? WHERE id = ?",
            now, 
            skill_id
        )
        .execute(self.pool).await?;
        Ok(())
    }
}