use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Biblioteca de funciones y prompts especializados que el agente puede invocar para resolver problemas específicos.
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