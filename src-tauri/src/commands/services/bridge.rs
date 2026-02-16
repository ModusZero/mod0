use tauri::{command, State, AppHandle};
use crate::services::bridge::registry::discovery::DiscoveryEngine;
use crate::services::bridge::registry::defs::RuntimeDefinition;
use crate::services::bridge::capabilities::manager::CapabilityManager;
use std::path::Path;

#[command]
pub async fn discover_workspace(project_path: String) -> Result<Vec<RuntimeDefinition>, String> {
    let engine = DiscoveryEngine::new(Path::new(&project_path));
    
    // Ejecuta la inferencia inteligente usando el categorizador
    let suggestions = engine.run_smart_inference();
    
    // Guarda el borrador en .mod0/capabilities.json
    engine.save_to_capabilities(&suggestions)?;
    
    Ok(suggestions)
}

#[command]
pub async fn boot_runtime(
    def: RuntimeDefinition, 
    manager: State<'_, CapabilityManager>, 
    app: AppHandle
) -> Result<(), String> {
    manager.spawn_runtime(def, app).await
}