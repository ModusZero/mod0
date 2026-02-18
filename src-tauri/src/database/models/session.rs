use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: String,
    pub project_path: String,
    pub session_type: String, 
    pub name: Option<String>,
    
    pub branch_name: Option<String>,
    pub worktree_path: Option<String>,
    
    pub status: String,
    pub created_at: DateTime<Utc>,
}