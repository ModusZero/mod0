use tauri::{command, AppHandle, State};
use crate::services::bridge::capabilities::mcp::McpManager;
use crate::services::bridge::codec::rpc_json::JsonRpcMessage;
use serde_json::Value;

#[command]
pub async fn send_mcp_request(
    server_name: String,
    method: String,
    params: Value,
    id: Option<Value>,
    project_path: String,
    manager: State<'_, McpManager>,
    app: AppHandle,
) -> Result<(), String> {
    
    // 1. Asegurar que el servidor MCP vive
    manager.ensure_server(&server_name, app, &project_path).await?;

    // 2. Enviar acción
    let mut servers = manager.servers.lock().await;
    if let Some(transport) = servers.get_mut(&server_name) {
        let msg = JsonRpcMessage {
            jsonrpc: "2.0".into(),
            id,
            method: Some(method),
            params: Some(params),
            result: None,
            error: None,
        };
        transport.send(msg).await?;
    }
    Ok(())
}