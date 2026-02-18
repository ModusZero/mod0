use super::Repository;
use crate::database::models::workspace::{Project, EditorState};
use chrono::{DateTime, Utc};

impl<'a> Repository<'a> {
    pub async fn upsert_project(&self, project: &Project) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO project (path, last_opened, is_favorite, custom_settings) 
               VALUES (?, ?, ?, ?)
               ON CONFLICT(path) DO UPDATE SET 
               last_opened = excluded.last_opened,
               is_favorite = excluded.is_favorite,
               custom_settings = excluded.custom_settings"#,
            project.path, project.last_opened, project.is_favorite, project.custom_settings
        )
        .execute(self.pool).await?;
        Ok(())
    }

    pub async fn save_editor_state(&self, state: &EditorState) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO editor_state (project_path, open_tabs, active_tab, layout_config) 
               VALUES (?, ?, ?, ?)
               ON CONFLICT(project_path) DO UPDATE SET 
               open_tabs = excluded.open_tabs,
               active_tab = excluded.active_tab,
               layout_config = excluded.layout_config"#,
            state.project_path, state.open_tabs, state.active_tab, state.layout_config
        )
        .execute(self.pool).await?;
        Ok(())
    }

    pub async fn get_project(&self, path: &str) -> sqlx::Result<Option<Project>> {
        sqlx::query_as!(
            Project,
            r#"SELECT 
                path as "path!", 
                last_opened as "last_opened!: DateTime<Utc>", 
                is_favorite as "is_favorite!", 
                custom_settings 
            FROM project 
            WHERE path = ?"#,
            path
        )
        .fetch_optional(self.pool)
        .await
    }

    pub async fn get_editor_state(&self, project_path: &str) -> sqlx::Result<Option<EditorState>> {
        sqlx::query_as!(
            EditorState,
            r#"SELECT 
                project_path as "project_path!", 
                open_tabs as "open_tabs!", 
                active_tab, 
                layout_config 
            FROM editor_state 
            WHERE project_path = ?"#,
            project_path
        )
        .fetch_optional(self.pool)
        .await
    }
}