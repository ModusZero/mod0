use tauri::{command, AppHandle, State};
use crate::services::bridge::capabilities::lsp::LspManager;
use crate::services::bridge::codec::rpc_json::JsonRpcMessage;
use serde_json::Value;

#[command]
pub async fn send_lsp_request(
    lang: String,
    method: String,
    params: Value,
    id: Option<Value>,
    manager: State<'_, LspManager>,
    app: AppHandle,
) -> Result<(), String> {
    
    // 1. Asegurar que el servidor esté corriendo
    manager.get_or_spawn(&lang, app).await?;

    // 2. Enviar mensaje
    let mut transports = manager.transports.lock().await;
    if let Some(transport) = transports.get_mut(&lang) {
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