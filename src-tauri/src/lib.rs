mod config; 
use config::AppConfig;
use std::sync::Mutex;
use tauri::{State, Manager, AppHandle};

pub struct AppState {
    pub config: Mutex<AppConfig>,
}

#[tauri::command]
fn get_config(state: State<AppState>) -> AppConfig {
    let config = state.config.lock().unwrap();
    config.clone()
}

#[tauri::command]
fn update_config(
    app_handle: AppHandle, 
    state: State<AppState>, 
    new_config: AppConfig
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    *config = new_config;
    
    let config_dir = app_handle.path().app_config_dir().map_err(|e| e.to_string())?;
    config.save(config_dir)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            
            #[cfg(target_os = "windows")]
            {
                main_window.set_decorations(false).ok();
                main_window.set_shadow(true).ok();
            }

            let config_dir = app.path().app_config_dir().map_err(|e| e.to_string()).unwrap();
            let initial_config = AppConfig::load(config_dir);
            
            app.manage(AppState {
                config: Mutex::new(initial_config),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_config, update_config]) 
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}