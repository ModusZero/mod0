use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

// TODO redefinir para que en lugar de guardarlo en una BD,
// se guarde el ultimo estado en temp_files en md dentro del mismo workspace cada cierto tiempo

/// Define la autoridad y origen de los mensajes en el historial (Usuario, Asistente o Sistema).
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum ChatRole {
    User,
    Assistant,
    System,
}

/// Registro cronológico de la comunicación entre el usuario y la IA dentro de un proyecto y sesión específicos.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChatHistory {
    pub id: i64,
    pub project_path: String,
    pub session_id: String,
    pub role: ChatRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}