use chrono::{DateTime, Utc};
use super::terminal_history::{TerminalHistory};
use super::Repository;

impl<'a> Repository<'a> {
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

    /// Elimina el historial de terminal de una sesión.
    pub async fn clear_terminal_history(&self, session_id: &str) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM terminal_history WHERE session_id = ?", session_id)
            .execute(self.pool).await?;
        Ok(())
    }
}