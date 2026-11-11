use tauri::State;

use crate::core::error::AppResult;
use crate::core::state::AppState;
use crate::features::observer::graph::ThoughtNode;
use crate::features::sessions::model::{CreateSessionRequest, Session};
use crate::infrastructure::process::blocking;

#[tauri::command]
pub async fn session_list(state: State<'_, AppState>) -> AppResult<Vec<Session>> {
    Ok(state.sessions.list())
}

#[tauri::command]
pub async fn session_create(state: State<'_, AppState>, request: CreateSessionRequest) -> AppResult<Session> {
    let sessions = state.sessions.clone();
    blocking(move || sessions.create(request)).await
}

#[tauri::command]
pub async fn session_restart(state: State<'_, AppState>, id: String) -> AppResult<Session> {
    let sessions = state.sessions.clone();
    blocking(move || sessions.restart(&id)).await
}

#[tauri::command]
pub async fn session_stop(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.sessions.stop(&id)
}

#[tauri::command]
pub async fn session_remove(state: State<'_, AppState>, id: String, remove_worktree: bool) -> AppResult<()> {
    let sessions = state.sessions.clone();
    blocking(move || sessions.remove(&id, remove_worktree)).await
}

#[tauri::command]
pub async fn session_rename(state: State<'_, AppState>, id: String, name: String) -> AppResult<Session> {
    state.sessions.rename(&id, &name)
}

#[tauri::command]
pub async fn session_set_reviewed(
    state: State<'_, AppState>,
    id: String,
    reviewed: bool,
) -> AppResult<Session> {
    state.sessions.set_reviewed(&id, reviewed)
}

#[tauri::command]
pub async fn session_graph(state: State<'_, AppState>, id: String) -> AppResult<Vec<ThoughtNode>> {
    state.sessions.graph(&id)
}

#[tauri::command]
pub async fn session_refresh(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.sessions.refresh_changes(&id);
    Ok(())
}
