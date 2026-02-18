mod config;
mod commands;
mod database;
mod services;
mod kernel;

use std::sync::Mutex;
use std::path::PathBuf;
use tauri::Manager;
use crate::config::AppConfig;
use crate::database::DbManager;
use crate::kernel::fs::worker::{IndexingWorker, IndexingTask};
use crate::kernel::terminal::TerminalManager;
use crate::services::bridge::BridgeService;

// Importación de comandos
use crate::commands::agent::*;
use crate::commands::kernel::{fs::*, terminal::*};
use crate::commands::services::{bridge::*, orchestration::*};
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
            
            // Determinar la raíz del proyecto para el BridgeService
            let project_root = last_project
                .as_ref()
                .map(PathBuf::from)
                .unwrap_or_else(|| config_dir.clone());

            // Inicializamos el BridgeService (que internamente crea los Managers)
            let bridge_service = BridgeService::new(project_root);
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

                // Registramos los servicios en el estado de Tauri
                handle.manage(db_manager);
                handle.manage(indexer_tx);
                handle.manage(terminal_manager);
                handle.manage(bridge_service); // Vital para los comandos de bridge y orchestration
                handle.manage(AppState {
                    config: Mutex::new(initial_config),
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // --- Configuración ---
            get_config,
            update_config,

            // *** Kernel ***
            
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

            //  *** Services ***

            // --- Bridge (Comandos directos al BridgeService) ---
            boot_runtime,
            stop_runtime,
            load_config,
            register_provider,
            activate_provider,
            ai_completion,
            vcs_action,

            //  --- Orchestration ---
            discover_recommendations_for_project,

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running MOD0");
}