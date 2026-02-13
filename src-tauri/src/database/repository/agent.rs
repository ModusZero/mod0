use super::Repository;
use crate::database::models::agent::{ThoughtNode, ExecutionTask};
use chrono::{DateTime, Utc};

impl<'a> Repository<'a> {
    pub async fn add_thought(&self, node: &ThoughtNode) -> sqlx::Result<i64> {
        let res = sqlx::query!(
            "INSERT INTO thought_node (session_id, parent_id, node_type, content, metadata) VALUES (?, ?, ?, ?, ?)",
            node.session_id, node.parent_id, node.node_type, node.content, node.metadata
        ).execute(self.pool).await?;
        Ok(res.last_insert_rowid())
    }

    pub async fn get_session_thoughts(&self, session_id: &str) -> sqlx::Result<Vec<ThoughtNode>> {
        sqlx::query_as!(
            ThoughtNode,
            r#"SELECT 
                id as "id!", 
                session_id as "session_id!", 
                parent_id, 
                node_type as "node_type!", 
                content as "content!", 
                metadata, 
                created_at as "created_at!: DateTime<Utc>" 
            FROM thought_node WHERE session_id = ? ORDER BY created_at ASC"#,
            session_id
        ).fetch_all(self.pool).await
    }

    pub async fn upsert_task(&self, task: &ExecutionTask) -> sqlx::Result<()> {
        sqlx::query!(
            r#"INSERT INTO execution_tasks (session_id, title, status, tdd_status, position) 
               VALUES (?, ?, ?, ?, ?)
               ON CONFLICT(id) DO UPDATE SET 
               status = excluded.status, 
               tdd_status = excluded.tdd_status, 
               position = excluded.position"#,
            task.session_id, task.title, task.status, task.tdd_status, task.position
        ).execute(self.pool).await?;
        Ok(())
    }
}