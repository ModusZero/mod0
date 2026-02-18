use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

/// Registro de interacciones en el chat, incluyendo artefactos generados por IA.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatHistory {
    pub id: i64,
    /// Ruta del proyecto para filtrado de contexto.
    pub project_path: String,
    /// Identificador único de la sesión/hilo actual.
    pub session_id: String, 
    /// "user" o "assistant".
    pub role: String,
    pub content: String,
    /// Datos serializados de artefactos (diagramas, tablas, UI prototypes).
    pub artifact_data: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Registro de comandos ejecutados en la terminal para auditoría y aprendizaje.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TerminalHistory {
    pub id: i64,
    pub project_path: String,
    pub session_id: String,
    pub command: String,
    pub exit_code: Option<i64>,
    pub timestamp: DateTime<Utc>,
}