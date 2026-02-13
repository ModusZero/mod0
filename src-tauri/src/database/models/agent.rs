use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ThoughtNode {
    pub id: i64,
    pub session_id: String,
    pub parent_id: Option<i64>,
    pub node_type: String,
    pub content: String,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ExecutionTask {
    pub id: i64,
    pub session_id: String,
    pub title: String,
    pub status: String,
    pub tdd_status: String,
    pub position: i32,
}