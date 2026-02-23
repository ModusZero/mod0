use sqlx::{QueryBuilder, Sqlite};
use chrono::{DateTime, Utc};
use super::skill::Skill;
use super::Repository;

impl<'a> Repository<'a> {
    /// Registra o actualiza una habilidad mediante una operación UPSERT.
    /// 
    /// ### Parámetros
    /// * `skill`: Estructura con la definición de la habilidad.
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

    /// Recupera el catálogo completo de habilidades registradas.
    /// 
    /// ### Retorna
    /// Un vector con todas las estructuras `Skill`.
    pub async fn get_all_skills(&self) -> sqlx::Result<Vec<Skill>> {
        sqlx::query_as!(
            Skill,
            r#"SELECT 
                id as "id!", 
                name as "name!", 
                code as "code!", 
                description, 
                tags,
                usage_count as "usage_count!",
                last_used as "last_used: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>"
               FROM skill"#
        )
        .fetch_all(self.pool).await
    }

    /// Filtra habilidades basadas en un conjunto de etiquetas (operación OR).
    /// 
    /// ### Parámetros
    /// * `tags`: Vector de cadenas de texto con las etiquetas a buscar.
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

    /// Incrementa el contador de uso y actualiza la marca temporal de una habilidad.
    /// 
    /// ### Parámetros
    /// * `skill_id`: Identificador único de la habilidad.
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