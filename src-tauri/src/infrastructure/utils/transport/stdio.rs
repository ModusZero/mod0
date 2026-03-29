use tokio::process::{ChildStdin, ChildStdout};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};
use tokio::sync::{Mutex, oneshot};
use std::sync::Arc;
use dashmap::DashMap;
use tauri::{AppHandle, Emitter};
use serde_json::Value;

// Almacén de respuestas pendientes: ID -> Sender para notificar al awaiter
type ResponseMap = Arc<DashMap<String, oneshot::Sender<Value>>>;

#[derive(Clone)]
pub struct StdioTransport {
    stdin: Arc<Mutex<ChildStdin>>,
    pending_responses: ResponseMap,
}

impl StdioTransport {
    pub fn new(stdin: ChildStdin) -> Self {
        Self {
            stdin: Arc::new(Mutex::new(stdin)),
            pending_responses: Arc::new(DashMap::new()),
        }
    }

    pub fn spawn_reader(&self, stdout: ChildStdout, event_channel: String, app: AppHandle) {
        let pending_responses = Arc::clone(&self.pending_responses);
        
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();

            loop {
                line.clear();
                if reader.read_line(&mut line).await.unwrap_or(0) == 0 { break; }
                
                let mut message_body: Option<Value> = None;

                if line.starts_with("Content-Length: ") {
                    if let Ok(len) = line["Content-Length: ".len()..].trim().parse::<usize>() {
                        let mut separator = String::new();
                        let _ = reader.read_line(&mut separator).await; // Leer el \r\n
                        let mut body = vec![0u8; len];
                        if reader.read_exact(&mut body).await.is_ok() {
                            message_body = serde_json::from_slice(&body).ok();
                        }
                    }
                } else {
                    // Si es una línea simple de JSON
                    message_body = serde_json::from_str(&line).ok();
                }

                if let Some(json) = message_body {
                    // Si el mensaje tiene un "id" y no tiene "method", es una RESPUESTA a algo que enviamos
                    if let Some(id_val) = json.get("id") {
                        let id_str = id_val.to_string();
                        if json.get("method").is_none() {
                            if let Some((_, tx)) = pending_responses.remove(&id_str) {
                                let _ = tx.send(json);
                                continue; // No emitir a la UI si es una respuesta interna capturada
                            }
                        }
                    }
                    
                    // Si no es una respuesta interna, es un evento/notificación para el frontend
                    let _ = app.emit(&event_channel, json);
                }
            }
        });
    }

    pub async fn send_and_wait(&self, id: String, data: Vec<u8>) -> Result<Value, String> {
        let (tx, rx) = oneshot::channel();
        
        // CORRECCIÓN: Clonamos 'id' para que el original siga vivo en este scope
        self.pending_responses.insert(id.clone(), tx);

        {
            let mut stdin = self.stdin.lock().await;
            stdin.write_all(&data).await.map_err(|e| e.to_string())?;
            stdin.flush().await.map_err(|e| e.to_string())?;
        }

        match tokio::time::timeout(std::time::Duration::from_secs(30), rx).await {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => Err("Canal de respuesta cerrado prematuramente".to_string()),
            Err(_) => {
                // Ahora podemos usar 'id' aquí porque no se movió en el insert
                self.pending_responses.remove(&id);
                Err("Timeout esperando respuesta de la extensión".to_string())
            }
        }
    }
}