use super::Repository;
use crate::database::models::core::Skill;

impl<'a> Repository<'a> {
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

    pub async fn get_skills_by_tag(&self, tag: &str) -> sqlx::Result<Vec<Skill>> {
        // SOLUCIÓN: El let binding hace que el String viva lo suficiente
        let pattern = format!("%{}%", tag);
        
        sqlx::query_as!(
            Skill,
            r#"SELECT id, name, code, description, tags FROM skills WHERE tags LIKE ?"#,
            pattern
        )
        .fetch_all(self.pool)
        .await
    }
}