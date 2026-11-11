use serde::Serialize;
use tauri::{AppHandle, State};

use crate::core::error::AppResult;
use crate::core::state::AppState;
use crate::features::notifications::{Notifier, NotifyKind};
use crate::features::sessions::launcher::detect_shell;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    pub hook_port: Option<u16>,
    pub data_dir: String,
    pub config_dir: String,
    pub shell: String,
}

#[tauri::command]
pub async fn app_info(state: State<'_, AppState>) -> AppResult<AppInfo> {
    let (shell, _, _) = detect_shell(&state.config.get());

    Ok(AppInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        hook_port: state.sessions.hook_port(),
        data_dir: state.paths.data_dir.to_string_lossy().into_owned(),
        config_dir: state.paths.config_dir.to_string_lossy().into_owned(),
        shell,
    })
}

/// Envía una notificación de prueba por todos los canales habilitados.
#[tauri::command]
pub async fn notify_test(app: AppHandle, state: State<'_, AppState>) -> AppResult<()> {
    Notifier::new(app, state.config.clone()).send(
        NotifyKind::Test,
        "MOD0",
        "Notificación de prueba: si lees esto, el canal funciona.",
        "prueba",
        "idle",
    );
    Ok(())
}
