use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatHistory {
    pub id: i64,
    pub project_path: String,
    pub session_id: String, 
    pub role: String,
    pub content: String,
    pub artifact_data: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TerminalHistory {
    pub id: i64,
    pub project_path: String,
    pub session_id: String,
    pub command: String,
    pub exit_code: Option<i64>,
    pub timestamp: DateTime<Utc>,
}