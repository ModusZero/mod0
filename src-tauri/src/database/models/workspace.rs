use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub path: String,
    pub last_opened: DateTime<Utc>,
    pub is_favorite: bool,
    pub custom_settings: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EditorState {
    pub project_path: String,
    pub open_tabs: String,
    pub active_tab: Option<String>,
    pub layout_config: Option<String>,
}