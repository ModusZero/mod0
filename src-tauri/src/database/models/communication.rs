use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: i64,
    pub project_path: String,
    pub session_id: Option<String>,
    pub role: String,
    pub content: String,
    pub artifact_data: Option<String>,
    pub timestamp: DateTime<Utc>,
}