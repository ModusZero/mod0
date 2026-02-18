use std::path::PathBuf;

use crate::database::{DbManager, models::files::Skill};

pub struct SearchService;

impl SearchService {
    pub async fn find_code(db: &DbManager, query: &str, project_path: &str) -> Result<Vec<PathBuf>, String> {
        let repo = db.repository();
        // Usamos el método que creamos en el repositorio que filtra por project_path
        repo.search_code_in_project(query, project_path)
            .await
            .map(|paths| paths.into_iter().map(PathBuf::from).collect())
            .map_err(|e| e.to_string())
    }

    pub async fn find_filename(db: &DbManager, query: &str, project_path: &str) -> Result<Vec<PathBuf>, String> {
        let pattern = format!("%{}%", query);
        
        let rows = sqlx::query!(
            r#"SELECT path as "path: String" FROM file_index 
               WHERE project_path = ? AND path LIKE ? LIMIT 50"#,
            project_path,
            pattern
        )
        .fetch_all(&db.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().filter_map(|r| r.path).map(PathBuf::from).collect())
    }

    pub async fn find_skills(db: &DbManager, tags: Vec<String>) -> Result<Vec<Skill>, String> {
        let repo = db.repository();
        repo.get_skills_by_tags(tags)
            .await
            .map_err(|e| e.to_string())
    }
}