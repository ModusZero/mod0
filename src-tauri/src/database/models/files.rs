use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct FileIndex {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FileMetadata {
    pub path: String,
    pub last_modified: DateTime<Utc>,
    pub size: i64,
    pub file_type: Option<String>,
    pub language_id: Option<String>,
    pub project_path: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Skill {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub usage_count: i64,
    pub last_used: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
