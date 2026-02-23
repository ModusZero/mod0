use reqwest::Client;
use serde_json::Value;

pub struct HttpTransport {
    client: Client,
    base_url: String,
}

impl HttpTransport {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// Envía un JSON-RPC o petición REST vía POST
    pub async fn post(&self, path: &str, body: Value) -> Result<Value, String> {
        let url = format!("{}{}", self.base_url, path);
        let res = self.client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        res.json::<Value>().await.map_err(|e| e.to_string())
    }

    /// Para interactuar con APIs que devuelven streams (muy importante para AI)
    pub async fn post_stream(&self, path: &str, body: Value) -> Result<reqwest::Response, String> {
        let url = format!("{}{}", self.base_url, path);
        self.client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())
    }
}