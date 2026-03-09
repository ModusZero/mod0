use std::path::PathBuf;

use crate::persistence::{DbManager, models::intelligence::skill::Skill};

pub struct FileSearch;

impl FileSearch {
    pub async fn find_code(db: &DbManager, query: &str, project_path: &str) -> Result<Vec<PathBuf>, String> {
        let repo = db.repository();
        repo.search_code_in_project(query, project_path)
            .await
            .map(|paths| paths.into_iter().map(PathBuf::from).collect())
            .map_err(|e| e.to_string())
    }

    pub async fn find_filename(db: &DbManager, query: &str, project_path: &str) -> Result<Vec<PathBuf>, String> {
        let repo = db.repository();
        repo.search_filename(query, project_path)
            .await
            .map(|paths| paths.into_iter().map(PathBuf::from).collect())
            .map_err(|e| e.to_string())
    }

    pub async fn find_skills_by_tags(db: &DbManager, tags: Vec<String>) -> Result<Vec<Skill>, String> {
        let repo = db.repository();
        repo.get_skills_by_tags(tags)
            .await
            .map_err(|e| e.to_string())
    }
}