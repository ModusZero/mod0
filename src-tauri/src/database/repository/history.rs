use super::Repository;
use crate::database::models::bridge::{ChatHistory, TerminalHistory, ChatRole};
use chrono::{DateTime, Utc};

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

    /// Registra una ejecución de comando en la terminal.
    /// 
    /// ### Parámetros
    /// * `project`: Ruta del proyecto.
    /// * `session_id`: Sesión activa.
    /// * `cmd`: Comando ejecutado.
    /// * `exit_code`: Código de salida (opcional).
    pub async fn add_terminal_entry(
        &self,
        project: &str,
        session_id: &str,
        cmd: &str,
        exit_code: Option<i64>
    ) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO terminal_history (project_path, session_id, command, exit_code) VALUES (?, ?, ?, ?)",
            project, session_id, cmd, exit_code
        ).execute(self.pool).await?;
        Ok(())
    }

    /// Obtiene el historial de terminal para análisis de contexto.
    pub async fn get_session_terminal(&self, session_id: &str) -> sqlx::Result<Vec<TerminalHistory>> {
        sqlx::query_as!(
            TerminalHistory,
            r#"SELECT 
                id as "id!", project_path as "project_path!", session_id as "session_id!", 
                command as "command!", exit_code as "exit_code: i64", 
                timestamp as "timestamp!: DateTime<Utc>" 
            FROM terminal_history WHERE session_id = ? ORDER BY timestamp ASC"#,
            session_id
        ).fetch_all(self.pool).await
    }

    /// Elimina el historial de chat de una sesión.
    pub async fn clear_chat_history(&self, session_id: &str) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM chat_history WHERE session_id = ?", session_id)
            .execute(self.pool).await?;
        Ok(())
    }

    /// Elimina el historial de terminal de una sesión.
    pub async fn clear_terminal_history(&self, session_id: &str) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM terminal_history WHERE session_id = ?", session_id)
            .execute(self.pool).await?;
        Ok(())
    }
}