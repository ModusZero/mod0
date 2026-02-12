use super::Repository;
use crate::database::models::core::Skill;
use sqlx::{QueryBuilder, Sqlite};

impl<'a> Repository<'a> {
    /// Inserta una nueva habilidad o actualiza una existente si el ID ya existe.
    pub async fn upsert_skill(&self, skill: &Skill) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO skills (id, name, code, description, tags) 
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
            "SELECT id, name, code, description, tags FROM skills WHERE "
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