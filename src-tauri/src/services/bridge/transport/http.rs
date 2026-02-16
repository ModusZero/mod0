use reqwest::Client;
use serde_json::Value;

pub struct HttpTransport {
    client: Client,
    endpoint: String,
}

impl HttpTransport {
    pub fn new(endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.to_string(),
        }
    }

    /// Envía un JSON-RPC vía POST (Común en servidores remotos)
    pub async fn post_rpc(&self, msg: Value) -> Result<Value, String> {
        let res = self.client.post(&self.endpoint)
            .json(&msg)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        res.json::<Value>().await.map_err(|e| e.to_string())
    }

    /// Para descargar bytes (VCS / Archivos)
    pub async fn get_raw(&self) -> Result<Vec<u8>, String> {
        let res = self.client.get(&self.endpoint)
            .send()
            .await
            .map_err(|e| e.to_string())?;
            
        let bytes = res.bytes().await.map_err(|e| e.to_string())?;
        Ok(bytes.to_vec())
    }
}