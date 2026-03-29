use tauri::{command, State};
use std::path::PathBuf;
use crate::services::bridge::BridgeService;
use crate::services::orchestration::discovery::{DiscoveryEngine, DiscoveryResponse};

#[command]
pub async fn discover_recommendations_for_project(
    project_root: String,
    service: State<'_, BridgeService>
) -> Result<DiscoveryResponse, String> {
    let path = PathBuf::from(project_root);
    let engine = DiscoveryEngine::new(&path);
    
    // Ejecuta el flujo: Scanner -> IA -> RuntimeDefinitions
    engine.discover_recommendations_for_workspace(&service).await
}