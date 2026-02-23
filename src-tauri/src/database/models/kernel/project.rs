use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Espacio de trabajo indexado, almacenando su ubicación física y preferencias globales.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub path: String,
    pub last_opened: DateTime<Utc>,
    pub is_favorite: bool,
    pub custom_settings: Option<String>,
}