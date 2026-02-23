use serde::{Deserialize, Serialize};
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

/// Representa una unidad de trabajo ejecutable dentro de una sesión, con seguimiento de errores y posición en la cola.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub session_id: String,
    pub skill_id: Option<i64>,
    pub title: String,
    pub status: TaskStatus,
    pub tdd_status: TddStatus,
    pub error_log: Option<String>,
    pub position: i64,
}