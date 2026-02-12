use crate::AppState;
use crate::config::AppConfig;
use tauri::{State, AppHandle, Manager};

#[tauri::command]
pub fn get_config(state: State<AppState>) -> AppConfig {
    state.config.lock().unwrap().clone()
}

#[tauri::command]
pub fn update_config(
    app_handle: AppHandle, 
    state: State<AppState>, 
    new_config: AppConfig
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    *config = new_config;
    let config_dir = app_handle.path().app_config_dir().map_err(|e| e.to_string())?;
    config.save(config_dir)
}