use super::Repository;
use crate::database::models::history::{ChatHistory, TerminalHistory};
use chrono::{DateTime, Utc};

impl<'a> Repository<'a> {
    pub async fn save_chat_message(&self, msg: &ChatHistory) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO chat_history (project_path, session_id, role, content, artifact_data) VALUES (?, ?, ?, ?, ?)",
            msg.project_path, msg.session_id, msg.role, msg.content, msg.artifact_data
        ).execute(self.pool).await?;
        Ok(())
    }

    pub async fn get_session_chat(&self, session_id: &str) -> sqlx::Result<Vec<ChatHistory>> {
        sqlx::query_as!(
            ChatHistory,
            r#"SELECT 
                id as "id!", 
                project_path as "project_path!", 
                session_id as "session_id!", 
                role as "role!", 
                content as "content!", 
                artifact_data, 
                timestamp as "timestamp!: DateTime<Utc>" 
            FROM chat_history WHERE session_id = ? ORDER BY timestamp ASC"#,
            session_id
        ).fetch_all(self.pool).await
    }

    pub async fn add_terminal_entry(&self, project: &str, session_id: &str, cmd: &str, exit_code: Option<i64>) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO terminal_history (project_path, session_id, command, exit_code) VALUES (?, ?, ?, ?)",
            project, session_id, cmd, exit_code
        ).execute(self.pool).await?;
        Ok(())
    }

    pub async fn get_session_terminal(&self, session_id: &str) -> sqlx::Result<Vec<TerminalHistory>> {
        sqlx::query_as!(
            TerminalHistory,
            r#"SELECT 
                id as "id!", 
                project_path as "project_path!", 
                session_id as "session_id!", 
                command as "command!", 
                exit_code, 
                timestamp as "timestamp!: DateTime<Utc>" 
            FROM terminal_history WHERE session_id = ? ORDER BY timestamp ASC"#,
            session_id
        ).fetch_all(self.pool).await
    }
}