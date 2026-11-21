use serde_json::Value;
use tauri::State;

use crate::core::error::AppResult;
use crate::core::state::AppState;
use crate::features::kanban;

#[tauri::command]
pub async fn kanban_load(state: State<'_, AppState>, project: String) -> AppResult<Value> {
    kanban::load(&state.paths, &project)
}

#[tauri::command]
pub async fn kanban_save(state: State<'_, AppState>, project: String, board: Value) -> AppResult<()> {
    kanban::save(&state.paths, &project, &board)
}
