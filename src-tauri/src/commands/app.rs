use std::collections::BTreeMap;

use serde::Serialize;
use tauri::{AppHandle, State};

use crate::core::error::AppResult;
use crate::core::state::AppState;
use crate::features::notifications::{Notifier, NotifyKind};
use crate::features::sessions::launcher::{detect_shell, env_reference, find_executable};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    pub hook_port: Option<u16>,
    pub data_dir: String,
    pub config_dir: String,
    pub shell: String,
    /// Ruta de `git`, si está instalado (lo necesitan la revisión y los worktrees).
    pub git: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandCheck {
    pub found: bool,
    pub path: Option<String>,
    /// Variables `$NOMBRE` del perfil que no existen en el entorno del sistema.
    pub missing_env: Vec<String>,
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
        git: find_executable("git").map(|path| path.to_string_lossy().into_owned()),
    })
}

/// Comprueba si el ejecutable de un perfil está instalado y visible en el `PATH`.
#[tauri::command]
pub async fn profile_check(command: String, env: BTreeMap<String, String>) -> AppResult<CommandCheck> {
    let path = find_executable(&command);
    let mut missing_env: Vec<String> = env
        .values()
        .filter_map(|value| env_reference(value))
        .filter(|name| std::env::var(name).map(|v| v.is_empty()).unwrap_or(true))
        .map(String::from)
        .collect();
    missing_env.sort();
    missing_env.dedup();

    Ok(CommandCheck {
        found: path.is_some(),
        path: path.map(|p| p.to_string_lossy().into_owned()),
        missing_env,
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
