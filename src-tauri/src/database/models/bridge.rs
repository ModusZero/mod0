use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Estados de control de tareas, permitiendo seguir el progreso de tareas bajo metodologías de desarrollo guiado por pruebas.
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "PascalCase")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Check,
    Done,
}

/// Estados de control de pruebas, permitiendo seguir el progreso de tareas bajo metodologías de desarrollo guiado por pruebas.
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum TddStatus {
    Red,
    Green,
    Refactor,
}

/// Define la autoridad y origen de los mensajes en el historial (Usuario, Asistente o Sistema).
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum ChatRole {
    User,
    Assistant,
    System,
}

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

/// Representa una unidad de trabajo ejecutable dentro de una sesión, con seguimiento de errores y posición en la cola.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ExecutionTask {
    pub id: i64,
    pub session_id: String,
    pub skill_id: Option<i64>,
    pub title: String,
    pub status: TaskStatus,
    pub tdd_status: TddStatus,
    pub error_log: Option<String>,
    pub position: i64,
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