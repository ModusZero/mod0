use chrono::{DateTime, Utc};
use super::session::{Session, SessionType};
use super::status::{Status};
use super::Repository;

impl<'a> Repository<'a> {
    /// Registra una nueva sesión de trabajo vinculada a un proyecto específico.
    ///
    /// ### Parámetros
    /// * `session`: Estructura con la configuración de la sesión.
    pub async fn create_session(&self, session: &Session) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO session (
                id, project_path, session_type, name, branch_name, 
                worktree_path, status, is_ghost_session, base_commit_hash
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            session.id,
            session.project_path,
            session.session_type,
            session.name,
            session.branch_name,
            session.worktree_path,
            session.status,
            session.is_ghost_session,
            session.base_commit_hash
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    /// Recupera la configuración y el estado actual de una sesión mediante su identificador.
    ///
    /// ### Parámetros
    /// * `id`: Identificador único de la sesión (UUID o hash).
    pub async fn get_session(&self, id: &str) -> sqlx::Result<Option<Session>> {
        sqlx::query_as!(
            Session,
            r#"SELECT 
                id as "id!", 
                project_path as "project_path!", 
                session_type as "session_type: SessionType", 
                name, 
                branch_name, 
                worktree_path, 
                status as "status: Status", 
                is_ghost_session as "is_ghost_session!", 
                base_commit_hash, 
                created_at as "created_at!: DateTime<Utc>" 
            FROM session 
            WHERE id = ?"#,
            id,
        )
        .fetch_optional(self.pool)
        .await
    }

    /// Recupera la configuración y el estado actual de todas las sesiones a partir del `project_path`.
    /// 
    /// ### Parámetros
    /// * `project_path`: Identificador único del proyecto abierto.
    pub async fn get_all_sessions(&self, project_path: &str) -> sqlx::Result<Vec<Session>> {
        sqlx::query_as!(
            Session,
            r#"SELECT 
                id as "id!", 
                project_path as "project_path!", 
                session_type as "session_type: SessionType", 
                name, 
                branch_name, 
                worktree_path, 
                status as "status: Status", 
                is_ghost_session as "is_ghost_session!", 
                base_commit_hash, 
                created_at as "created_at!: DateTime<Utc>" 
            FROM session 
            WHERE project_path = ?
            ORDER BY created_at DESC"#,
            project_path
        )
        .fetch_all(self.pool).await
    }

    /// Actualiza el estado vital de la sesión (e.g. de Active a Completed).
    ///
    /// ### Parámetros
    /// * `id`: Identificador de la sesión.
    /// * `status`: Nuevo estado a persistir.
    pub async fn update_session_status(&self, id: &str, status: Status) -> sqlx::Result<()> {
        sqlx::query!(
            "UPDATE session SET status = ? WHERE id = ?",
            status,
            id
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }
}