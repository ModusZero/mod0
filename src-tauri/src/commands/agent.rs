use tauri::State;
use crate::database::DbManager;
use crate::database::models::agent::{ThoughtNode, ExecutionTask};
use crate::database::models::history::ChatHistory;

// --- PERSISTENCIA DE CHAT & ARTEFACTOS (Blueprint Mode) ---

/// Guarda un mensaje de chat. Si la IA genera un artefacto (diagrama, tabla, etc), 
/// este se guarda en `artifact_data` para ser reconstruido en el Blueprint Mode.
#[tauri::command]
pub async fn send_chat_message(
    db: State<'_, DbManager>,
    message: ChatHistory
) -> Result<(), String> {
    let repo = db.repository();
    repo.save_chat_message(&message)
        .await
        .map_err(|e| e.to_string())
}

/// Recupera los mensajes de la sesión para reconstruir el hilo de conversación
/// y el contexto visual de la IA.
#[tauri::command]
pub async fn get_chat_history(
    db: State<'_, DbManager>,
    session_id: String
) -> Result<Vec<ChatHistory>, String> {
    let repo = db.repository();
    repo.get_session_chat(&session_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn clear_chat_history(
    session_id: String, 
    db: State<'_, DbManager>
) -> Result<(), String> {
    let repo = db.repository();
    repo.clear_chat_history(&session_id)
        .await
        .map_err(|e| e.to_string())
}

// --- GRAFO DE PENSAMIENTO & TAREAS (Forge Mode) ---

/// Registra un nuevo paso en el razonamiento interno de la IA.
#[tauri::command]
pub async fn add_agent_thought(
    db: State<'_, DbManager>,
    node: ThoughtNode
) -> Result<i64, String> {
    let repo = db.repository();
    repo.add_thought(&node)
        .await
        .map_err(|e| e.to_string())
}

/// Obtiene el grafo de pensamientos para visualizar la lógica del agente en tiempo real.
#[tauri::command]
pub async fn get_agent_session_history(
    db: State<'_, DbManager>,
    session_id: String
) -> Result<Vec<ThoughtNode>, String> {
    let repo = db.repository();
    repo.get_session_thoughts(&session_id)
        .await
        .map_err(|e| e.to_string())
}

/// Permite al usuario aprobar o rechazar un pensamiento/decisión de la IA.
#[tauri::command]
pub async fn update_thought_status(
    db: State<'_, DbManager>,
    id: i64,
    status: String
) -> Result<(), String> {
    let repo = db.repository();
    repo.update_thought_status(id, &status)
        .await
        .map_err(|e| e.to_string())
}

/// Sincroniza el estado de las tareas en el tablero Kanban del agente.
#[tauri::command]
pub async fn sync_execution_task(
    db: State<'_, DbManager>,
    task: ExecutionTask
) -> Result<(), String> {
    let repo = db.repository();
    repo.upsert_task(&task)
        .await
        .map_err(|e| e.to_string())
}

/// Reporta el resultado de una ejecución TDD para que la IA aprenda del error o confirme el éxito.
#[tauri::command]
pub async fn report_task_tdd_status(
    db: State<'_, DbManager>,
    id: i64,
    tdd_status: String,
    error_log: Option<String>
) -> Result<(), String> {
    let repo = db.repository();
    repo.update_task_tdd(id, &tdd_status, error_log)
        .await
        .map_err(|e| e.to_string())
}

/// Busca en la biblioteca de habilidades (Skills) las capacidades necesarias para el prompt actual.
#[tauri::command]
pub async fn discover_agent_skills(
    db: State<'_, DbManager>, 
    tags: Vec<String>
) -> Result<Vec<crate::database::models::files::Skill>, String> {
    let repo = db.repository();
    repo.get_skills_by_tags(tags)
        .await
        .map_err(|e| e.to_string())
}