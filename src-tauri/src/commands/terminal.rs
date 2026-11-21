use tauri::State;
use tauri::ipc::{Channel, InvokeResponseBody, Response};

use crate::core::error::AppResult;
use crate::core::state::AppState;

/// Conecta un canal de bytes crudos a la terminal de la sesión y devuelve su historial reciente
/// (bytes crudos) para reconstruirla al abrirla.
#[tauri::command]
pub async fn term_attach(
    state: State<'_, AppState>,
    id: String,
    channel: Channel<InvokeResponseBody>,
) -> AppResult<Response> {
    Ok(Response::new(state.sessions.pty(&id)?.attach(channel)))
}

#[tauri::command]
pub async fn term_detach(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.sessions.pty(&id)?.detach();
    Ok(())
}

#[tauri::command]
pub async fn term_write(state: State<'_, AppState>, id: String, data: String) -> AppResult<()> {
    state.sessions.pty(&id)?.write(data.as_bytes())
}

#[tauri::command]
pub async fn term_resize(state: State<'_, AppState>, id: String, rows: u16, cols: u16) -> AppResult<()> {
    state.sessions.pty(&id)?.resize(rows.max(2), cols.max(2))
}
