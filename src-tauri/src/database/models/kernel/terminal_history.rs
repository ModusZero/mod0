use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Auditoría de comandos ejecutados, almacenando códigos de salida para aprendizaje y diagnóstico del sistema.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TerminalHistory {
    pub id: i64,
    pub project_path: String,
    pub session_id: String,
    pub command: String,
    pub exit_code: Option<i64>,
    pub timestamp: DateTime<Utc>,
}