use chrono::{DateTime, Utc};
use super::chat_history::{ChatHistory, ChatRole};
use super::Repository;

impl<'a> Repository<'a> {
    /// Almacena un mensaje de chat en el historial persistente.
    /// 
    /// ### Parámetros
    /// * `msg`: Estructura con los datos del mensaje (rol, contenido y vinculación).
    pub async fn save_chat_message(&self, msg: &ChatHistory) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO chat_history (project_path, session_id, role, content) VALUES (?, ?, ?, ?)",
            msg.project_path, msg.session_id, msg.role, msg.content
        ).execute(self.pool).await?;
        Ok(())
    }

    /// Recupera el historial de chat de una sesión ordenado cronológicamente.
    /// 
    /// ### Parámetros
    /// * `session_id`: Identificador de la sesión.
    pub async fn get_session_chat(&self, session_id: &str) -> sqlx::Result<Vec<ChatHistory>> {
        sqlx::query_as!(
            ChatHistory,
            r#"SELECT 
                id as "id!",
                project_path as "project_path!",
                session_id as "session_id!",
                role as "role!: ChatRole",
                content as "content!",
                timestamp as "timestamp!: DateTime<Utc>"
            FROM chat_history WHERE session_id = ? ORDER BY timestamp ASC"#,
            session_id
        ).fetch_all(self.pool).await
    }

    /// Elimina el historial de chat de una sesión.
    pub async fn clear_chat_history(&self, session_id: &str) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM chat_history WHERE session_id = ?", session_id)
            .execute(self.pool).await?;
        Ok(())
    }
}