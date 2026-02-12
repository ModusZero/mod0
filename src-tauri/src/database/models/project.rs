use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProjectMeta {
    pub path: String,
    pub last_opened: DateTime<Utc>,
    pub is_favorite: bool,
    pub custom_settings: Option<String>,
}