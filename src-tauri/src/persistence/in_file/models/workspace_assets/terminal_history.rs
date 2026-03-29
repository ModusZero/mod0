use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

// TODO redefinir para que en lugar de guardarlo en una BD,
// se guarden los logs en temp_files en txt dentro del mismo workspace

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