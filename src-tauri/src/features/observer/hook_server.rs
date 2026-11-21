use std::io::Read;
use std::sync::Arc;
use std::thread;

use serde_json::Value;
use tiny_http::{Header, Response, Server};

use crate::core::error::{AppError, AppResult};
use crate::features::sessions::manager::SessionManager;

/// Límite de tamaño de un evento (PostToolUse puede incluir contenido de archivos).
const MAX_BODY_BYTES: u64 = 8 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct HookInfo {
    pub port: u16,
    pub token: String,
}

impl HookInfo {
    pub fn url_for(&self, session_id: &str) -> String {
        format!(
            "http://127.0.0.1:{}/hook/{}/{}",
            self.port, session_id, self.token
        )
    }
}

/// Servidor HTTP local (solo 127.0.0.1) que recibe los hooks de Claude Code.
/// Responde de inmediato `{}` para no frenar a la CLI y procesa el evento después.
pub fn start(sessions: Arc<SessionManager>) -> AppResult<HookInfo> {
    let server = Server::http("127.0.0.1:0").map_err(|e| AppError::msg(format!("hook server: {e}")))?;
    let port = server
        .server_addr()
        .to_ip()
        .map(|addr| addr.port())
        .ok_or_else(|| AppError::msg("hook server sin dirección IP"))?;
    let info = HookInfo {
        port,
        token: uuid::Uuid::new_v4().simple().to_string(),
    };

    let token = info.token.clone();
    thread::Builder::new().name("mod0-hooks".into()).spawn(move || {
        for request in server.incoming_requests() {
            let (sessions, token) = (sessions.clone(), token.clone());
            thread::spawn(move || handle(request, &sessions, &token));
        }
    })?;

    Ok(info)
}

fn handle(mut request: tiny_http::Request, sessions: &Arc<SessionManager>, token: &str) {
    let url = request.url().to_string();
    let parts: Vec<&str> = url.trim_start_matches('/').split('/').collect();
    let authorized = parts.len() == 3 && parts[0] == "hook" && parts[2] == token;

    let mut body = String::new();
    if authorized {
        let _ = request.as_reader().take(MAX_BODY_BYTES).read_to_string(&mut body);
    }

    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).expect("static header");
    let response = Response::from_string("{}")
        .with_status_code(if authorized { 200 } else { 403 })
        .with_header(header);
    let _ = request.respond(response);

    if authorized {
        if let Ok(payload) = serde_json::from_str::<Value>(&body) {
            sessions.on_hook(parts[1], payload);
        }
    }
}
