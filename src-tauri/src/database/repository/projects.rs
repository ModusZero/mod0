use super::Repository;
use crate::database::models::communication::ChatMessage;
use chrono::{DateTime, Utc};

impl<'a> Repository<'a> {
    pub async fn save_message(&self, msg: &ChatMessage) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO chat_history (project_path, session_id, role, content, artifact_data) 
               VALUES (?, ?, ?, ?, ?)"#,
            msg.project_path, msg.session_id, msg.role, msg.content, msg.artifact_data
        )
        .execute(self.pool).await?;
        Ok(())
    }

    pub async fn get_project_history(&self, path: &str) -> sqlx::Result<Vec<ChatMessage>> {
        sqlx::query_as!(
            ChatMessage,
            r#"SELECT 
                id, 
                project_path, 
                session_id, 
                role, 
                content, 
                artifact_data, 
                timestamp as "timestamp: DateTime<Utc>"
               FROM chat_history 
               WHERE project_path = ? 
               ORDER BY timestamp ASC"#,
            path
        )
        .fetch_all(self.pool).await
    }
}