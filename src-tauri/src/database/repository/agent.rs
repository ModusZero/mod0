use super::Repository;
use crate::database::models::agent::{AgentSession, ThoughtNode, ExecutionTask};

impl<'a> Repository<'a> {
    // --- Sesiones ---
    pub async fn create_session(&self, session: &AgentSession) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO agent_sessions (id, project_path, branch_name, status) VALUES (?, ?, ?, ?)",
            session.id, session.project_path, session.branch_name, session.status
        )
        .execute(self.pool).await?;
        Ok(())
    }

    // --- Grafo de Pensamiento ---
    pub async fn add_thought(&self, node: &ThoughtNode) -> sqlx::Result<i64> {
        let res = sqlx::query!(
            "INSERT INTO thought_nodes (session_id, parent_id, node_type, content, metadata) VALUES (?, ?, ?, ?, ?)",
            node.session_id, node.parent_id, node.node_type, node.content, node.metadata
        )
        .execute(self.pool).await?;
        Ok(res.last_insert_rowid())
    }

    // --- Kanban / Tasks ---
    pub async fn upsert_task(&self, task: &ExecutionTask) -> sqlx::Result<()> {
        sqlx::query!(
            "INSERT INTO execution_tasks (id, session_id, title, status, tdd_status, position) 
             VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET status = excluded.status, tdd_status = excluded.tdd_status, position = excluded.position",
            task.id, task.session_id, task.title, task.status, task.tdd_status, task.position
        )
        .execute(self.pool).await?;
        Ok(())
    }
}