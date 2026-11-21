use tauri::State;

use crate::config::AppConfig;
use crate::core::error::AppResult;
use crate::core::state::AppState;

#[tauri::command]
pub async fn config_get(state: State<'_, AppState>) -> AppResult<AppConfig> {
    Ok(state.config.get())
}

#[tauri::command]
pub async fn config_update(state: State<'_, AppState>, config: AppConfig) -> AppResult<AppConfig> {
    state.config.replace(config)
}

#[tauri::command]
pub async fn config_set_project(state: State<'_, AppState>, path: String) -> AppResult<AppConfig> {
    state.config.touch_project(&path)
}
