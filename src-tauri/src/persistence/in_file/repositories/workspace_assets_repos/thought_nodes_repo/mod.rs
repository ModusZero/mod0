use chrono::{DateTime, Utc};
use super::thought_node::{ThoughtNode, NodeType};
use super::status::{Status};
use super::Repository;

impl<'a> Repository<'a> {
    /// Inserta un nuevo nodo de pensamiento en la sesión.
    /// 
    /// ### Parámetros
    /// * `node`: Estructura con los datos del pensamiento.
    /// 
    /// ### Retorna
    /// El ID autonumérico del nodo insertado.
    pub async fn add_thought(&self, node: &ThoughtNode) -> sqlx::Result<i64> {
        let res = sqlx::query!(
            "INSERT INTO thought_node (session_id, parent_id, node_type, content, status, metadata) 
             VALUES (?, ?, ?, ?, ?, ?)",
            node.session_id, node.parent_id, node.node_type, node.content, node.status, node.metadata
        ).execute(self.pool).await?;
        Ok(res.last_insert_rowid())
    }

    /// Recupera todos los pensamientos asociados a una sesión específica.
    /// 
    /// ### Parámetros
    /// * `session_id`: Identificador único de la sesión.
    pub async fn get_thoughts_by_session(&self, session_id: &str) -> sqlx::Result<Vec<ThoughtNode>> {
        sqlx::query_as!(
            ThoughtNode,
            r#"SELECT
                id as "id!", 
                session_id as "session_id!", 
                parent_id, 
                node_type as "node_type!: NodeType", 
                content as "content!", 
                status as "status!: Status", 
                metadata, 
                created_at as "created_at!: DateTime<Utc>"
            FROM thought_node 
            WHERE session_id = ?
            ORDER BY created_at ASC"#,
            session_id
        ).fetch_all(self.pool).await
    }

    /// Actualiza el estado de un nodo de pensamiento (e.g., de 'pending' a 'completed').
    pub async fn update_thought_status(&self, id: i64, status: &str) -> sqlx::Result<()> {
        sqlx::query!("UPDATE thought_node SET status = ? WHERE id = ?", status, id)
            .execute(self.pool).await?;
        Ok(())
    }
}