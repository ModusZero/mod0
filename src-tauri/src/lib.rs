mod commands;
mod config;
mod core;
mod features;
mod infrastructure;

use std::sync::Arc;

use tauri::Manager;

use crate::config::ConfigStore;
use crate::core::state::{AppState, Paths};
use crate::features::sessions::manager::SessionManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let handle = app.handle().clone();
            let paths = Paths::resolve(&handle)?;
            let config = Arc::new(ConfigStore::load(&paths));

            let sessions = SessionManager::new(handle, paths.clone(), config.clone());
            sessions.load_persisted();
            sessions.start_hook_server()?;
            sessions.start_ticker();

            app.manage(AppState {
                paths,
                config,
                sessions,
            });

            // En macOS se conservan los controles nativos de la ventana.
            #[cfg(target_os = "macos")]
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_decorations(true);
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                if let Some(state) = window.try_state::<AppState>() {
                    state.sessions.shutdown();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            // --- App ---
            commands::app::app_info,
            commands::app::notify_test,
            // --- Configuración ---
            commands::config::config_get,
            commands::config::config_update,
            commands::config::config_set_project,
            // --- Sesiones ---
            commands::session::session_list,
            commands::session::session_create,
            commands::session::session_restart,
            commands::session::session_stop,
            commands::session::session_remove,
            commands::session::session_rename,
            commands::session::session_set_reviewed,
            commands::session::session_graph,
            commands::session::session_refresh,
            // --- Terminal ---
            commands::terminal::term_attach,
            commands::terminal::term_detach,
            commands::terminal::term_write,
            commands::terminal::term_resize,
            // --- Archivos ---
            commands::workspace::fs_list_dir,
            commands::workspace::fs_list_files,
            commands::workspace::fs_read,
            commands::workspace::fs_write,
            commands::workspace::fs_create_file,
            commands::workspace::fs_create_dir,
            commands::workspace::fs_rename,
            commands::workspace::fs_delete,
            commands::workspace::fs_graph,
            // --- Revisión (git) ---
            commands::review::git_info,
            commands::review::review_changes,
            commands::review::review_file,
            commands::review::review_commit,
            // --- Kanban ---
            commands::kanban::kanban_load,
            commands::kanban::kanban_save,
        ])
        .run(tauri::generate_context!())
        .expect("error while running MOD0");
}
