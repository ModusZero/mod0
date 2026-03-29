use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

// TODO Definir todo esto de forma que en lugar de una tabla en la BD, 
// se divida por folders en temp_files dentro del mismo workspace.
// Que guarde en cada folder (ai_chats, todo_tasks, terminal_logs) un folder por cada session
// de la misma forma que hace cline, guardarlo en la bd complica las cosas sin necesidad, 
// y tampoco mejora la latencia, hacerlo en archivos es mas cercano al user y facil de editar y auditar

/// Define el contexto de ejecución de una sesión.
/// - `UserChat`: Sesión iniciada por un user al abrir un chat nuevo.
/// - `AgentTask`: Sesión iniciada de forma autónoma por el agente para subdividir sus tasks.
/// - `Terminal`: Sesión de terminal iniciada por el user o por el agent.
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum SessionType {
    UserChat,
    AgentTask,
    Terminal,
}

/// Contenedor principal de un hilo de trabajo; incluye soporte para multitasking en entornos controlados y "Ghost Sessions" destinadas a pruebas en sandboxes.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: String,
    pub project_path: String,
    pub session_type: SessionType,
    pub name: Option<String>,
    pub branch_name: Option<String>,
    pub worktree_path: Option<String>,
    pub status: super::super::shared::status::Status,
    pub is_ghost_session: bool,
    pub base_commit_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}