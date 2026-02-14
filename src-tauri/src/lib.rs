mod config;
mod commands;
mod database;
mod services;
mod kernel;

use std::sync::Mutex;
use std::path::PathBuf;
use tauri::Manager;
use crate::config::AppConfig;
use crate::services::bridge::capabilities::lsp::LspManager;
use crate::services::bridge::capabilities::mcp::McpManager;
use crate::database::DbManager;
use crate::kernel::fs::worker::{IndexingWorker, IndexingTask};
use crate::kernel::terminal::TerminalManager; // Importado correctamente
use tokio::sync::mpsc;

// Imports de comandos
use crate::commands::mcp::*;
use crate::commands::agent::*;
use crate::commands::lsp::*;
use crate::commands::kernel::fs::*;
use crate::commands::kernel::terminal::*;
use crate::commands::config::*;

/// Estado global de la aplicación para acceso rápido desde comandos
pub struct AppState {
    pub config: Mutex<AppConfig>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let handle = app.handle().clone();
            let config_dir = handle.path().app_config_dir().expect("No se pudo obtener config_dir");
            
            if !config_dir.exists() {
                std::fs::create_dir_all(&config_dir).unwrap();
            }

            let initial_config = AppConfig::load(config_dir.clone());
            let last_project = initial_config.last_project_path.clone();
            
            let terminal_manager = TerminalManager::new(handle.clone());

            tauri::async_runtime::block_on(async move {
                let db_path = config_dir.join("modus_zero.db");
                
                let db_manager = DbManager::new(&db_path)
                    .await
                    .expect("Fallo crítico al inicializar la base de datos");

                let indexer_tx = IndexingWorker::spawn(db_manager.clone());

                if let Some(path_str) = last_project {
                    let project_path = PathBuf::from(path_str);
                    if project_path.exists() {
                        let _ = indexer_tx.send(IndexingTask::FullScan(project_path)).await;
                    }
                }

                handle.manage(db_manager);
                handle.manage(indexer_tx);
                handle.manage(terminal_manager);
                handle.manage(AppState {
                    config: Mutex::new(initial_config),
                });
            });

            Ok(())
        })
        .manage(LspManager::new())
        .manage(McpManager::new())
        .invoke_handler(tauri::generate_handler![
            // --- Configuración ---
            get_config,
            update_config,

            // --- Filesystem ---
            get_file_tree,
            read_file,
            write_file,
            create_new_file,
            create_new_dir,
            rename_path,
            delete_path,
            global_search,
            quick_open,

            // --- Terminal ---
            term_create_session,
            term_write,
            term_resize,
            term_get_context,
            term_get_history,
            term_clear_history,

            // --- Agents ---
            send_chat_message,
            get_chat_history,
            clear_chat_history,
            add_agent_thought,
            get_agent_session_history,
            sync_execution_task,
            update_thought_status,
            report_task_tdd_status,
            discover_agent_skills,

            // --- Proxies de Protocolos ---
            send_lsp_request,
            send_mcp_request,
        ])
        .run(tauri::generate_context!())
        .expect("error while running MOD0");
}