use tauri::{command, State, AppHandle};
use serde_json::Value;
use crate::services::bridge::BridgeService;
use crate::services::bridge::defs::{RuntimeDefinition, ProviderConfig};

// --- RUNTIMES (MCP/LSP) ---

#[command]
pub async fn boot_runtime(
    def: RuntimeDefinition, 
    service: State<'_, BridgeService>, 
    app: AppHandle
) -> Result<(), String> {
    service.spawn_runtime(def, app).await
}

#[command]
pub async fn stop_runtime(
    id: String,
    service: State<'_, BridgeService>
) -> Result<(), String> {
    service.stop_runtime(&id)
}

#[command]
pub async fn load_config(
    service: State<'_, BridgeService>
) -> Result<Vec<RuntimeDefinition>, String> {
    Ok(service.load_runtime_configs())
}

// --- PROVIDERS (AI/VCS) ---

#[command]
pub async fn register_provider(
    config: ProviderConfig,
    service: State<'_, BridgeService>
) -> Result<(), String> {
    service.register_provider(config)
}

#[command]
pub async fn activate_provider(
    id: String,
    service: State<'_, BridgeService>
) -> Result<(), String> {
    service.activate_provider(&id).await
}

#[command]
pub async fn ai_completion(
    prompt: String,
    service: State<'_, BridgeService>
) -> Result<String, String> {
    service.ask_ai(&prompt).await
}

#[command]
pub async fn vcs_action(
    action: String, 
    args: Value,
    service: State<'_, BridgeService>
) -> Result<String, String> {
    service.execute_vcs_action(&action, args).await
}