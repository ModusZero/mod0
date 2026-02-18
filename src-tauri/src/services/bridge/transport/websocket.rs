use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::sync::mpsc::channel;
use tauri::{AppHandle, Emitter};

pub struct VsCodeTransport {
    tx: tokio::sync::mpsc::Sender<Message>,
}

impl VsCodeTransport {
    pub async fn connect(url: &str, event_channel: String, app: AppHandle) -> Result<Self, String> {
        let (ws_stream, _) = connect_async(
            url
        ).await.map_err(|e| e.to_string())?;
        let (mut write, mut read) = ws_stream.split();
        let (tx, mut rx) = channel::<Message>(100);

        // Task de Escritura
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let _ = write.send(msg).await;
            }
        });

        // Task de Lectura
        tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                if let Message::Text(text) = msg {
                    // Convertimos el Utf8Bytes a String para Tauri
                    let _ = app.emit(&event_channel, text.to_string());
                } else if let Message::Binary(bin) = msg {
                    let _ = app.emit(&format!("{}:raw", event_channel), bin.to_vec());
                }
            }
        });

        Ok(Self { tx })
    }

    pub async fn send_text(&self, text: String) -> Result<(), String> {
        self.tx.send(Message::Text(text.into())).await.map_err(|e| e.to_string())
    }
}