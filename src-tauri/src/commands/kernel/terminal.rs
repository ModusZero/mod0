use tauri::{State};
use crate::kernel::terminal::{TerminalManager, history::HistoryService};
use crate::database::models::history::{TerminalHistory};
use crate::database::DbManager;
use std::path::PathBuf;

#[tauri::command]
pub async fn term_create_session(
    id: String, 
    path: PathBuf, 
    manager: State<'_, TerminalManager>
) -> Result<(), String> {
    if !path.exists() { return Err("Ruta inválida".into()); }
    manager.create_session(id, path).await
}

#[tauri::command]
pub async fn term_get_context(id: String, manager: State<'_, TerminalManager>) -> Result<String, String> {
    // Permite a la IA leer qué hay actualmente en la terminal
    manager.get_capture(&id).await
}

#[tauri::command]
pub async fn term_resize(id: String, rows: u16, cols: u16, manager: State<'_, TerminalManager>) -> Result<(), String> {
    manager.resize(&id, rows, cols).await
}

#[tauri::command]
pub async fn term_write(
    id: String, 
    data: String, 
    is_ai: bool,
    manager: State<'_, TerminalManager>,
    db: State<'_, DbManager>
) -> Result<(), String> {
    
    // 1. Control de seguridad para IA
    if is_ai {
        // Aquí puedes consultar settings en el futuro
        println!("IA ejecutando: {}", data);
    }

    // 2. Ejecutar escritura
    let project_path = manager.write(&id, &data).await?;

    // 3. Persistencia asíncrona (si es un comando)
    if data.contains('\r') || data.contains('\n') {
        let db_clone = db.inner().clone();
        let id_clone = id.clone();
        let data_clone = data.clone();
        let path_str = project_path.to_string_lossy().to_string();

        tokio::spawn(async move {
            HistoryService::log_command(db_clone, id_clone, path_str, data_clone).await;
        });
    }

    Ok(())
}

#[tauri::command]
pub async fn term_get_history(
    session_id: String, 
    db: State<'_, DbManager>
) -> Result<Vec<TerminalHistory>, String> {
    let db_clone = db.inner().clone();
    HistoryService::get_history(db_clone, session_id).await
}

#[tauri::command]
pub async fn term_clear_history(
    session_id: String, 
    db: State<'_, DbManager>
) -> Result<(), String> {
    let db_clone = db.inner().clone();
    HistoryService::clear_history(db_clone, session_id).await
}