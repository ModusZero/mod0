use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Skill {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct FileIndex {
    pub path: String,
    pub content: String,
}