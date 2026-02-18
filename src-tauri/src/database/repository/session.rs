use super::Repository;
use crate::database::models::session::Session;
use chrono::{DateTime, Utc};

impl<'a> Repository<'a> {
    pub async fn create_session(&self, session: &Session) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO session (id, project_path, session_type, name, branch_name, worktree_path, status) 
               VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            session.id, 
            session.project_path, 
            session.session_type, 
            session.name,
            session.branch_name,
            session.worktree_path, 
            session.status
        )
        .execute(self.pool).await?;
        Ok(())
    }

    pub async fn get_session_by_id(&self, id: &str) -> sqlx::Result<Option<Session>> {
        sqlx::query_as!(
            Session,
            r#"SELECT 
                id as "id!", 
                project_path as "project_path!", 
                session_type as "session_type!", 
                name, 
                branch_name, 
                worktree_path, 
                status as "status!", 
                created_at as "created_at!: DateTime<Utc>" 
               FROM session WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool).await
    }

    pub async fn get_all_sessions(&self) -> sqlx::Result<Vec<Session>> {
        sqlx::query_as!(
            Session,
            r#"SELECT 
                id as "id!", 
                project_path as "project_path!", 
                session_type as "session_type!", 
                name, 
                branch_name, 
                worktree_path, 
                status as "status!", 
                created_at as "created_at!: DateTime<Utc>" 
               FROM session ORDER BY created_at DESC"#
        )
        .fetch_all(self.pool).await
    }
}