use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::sync::mpsc::{channel, Sender};
use tauri::{AppHandle, Emitter};

pub struct WebSocketTransport {
    tx: Sender<Message>,
}

impl WebSocketTransport {
    pub async fn connect(url: &str, event_channel: String, app: AppHandle) -> Result<Self, String> {
        let (ws_stream, _) = connect_async(url).await.map_err(|e| e.to_string())?;
        let (mut write, mut read) = ws_stream.split();
        let (tx, mut rx) = channel::<Message>(100);

        // Task: Outgoing (Escritura)
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let _ = write.send(msg).await;
            }
        });

        // Task: Incoming (Lectura)
        tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                match msg {
                    Message::Text(text) => {
                        let _ = app.emit(&event_channel, text.to_string());
                    },
                    Message::Binary(bin) => {
                        let _ = app.emit(&format!("{}:raw", event_channel), bin.to_vec());
                    },
                    _ => {}
                }
            }
        });

        Ok(Self { tx })
    }

    pub async fn send(&self, text: &str) -> Result<(), String> {
        self.tx.send(Message::Text(text.into())).await.map_err(|e| e.to_string())
    }
}