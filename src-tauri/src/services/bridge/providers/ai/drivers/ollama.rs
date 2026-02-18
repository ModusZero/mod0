use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use crate::services::bridge::{defs::ProviderConfig, providers::ai::AiProvider};

pub struct OllamaProvider {
    config: ProviderConfig,
    client: Client,
}

impl OllamaProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self { config, client: Client::new() }
    }
}

#[async_trait]
impl AiProvider for OllamaProvider {
    async fn completion(&self, prompt: &str) -> Result<String, String> {
        let url = self.config.endpoint.as_deref().unwrap_or("http://localhost:11434/api/generate");
        let model = self.config.settings["model"].as_str().unwrap_or("llama3");

        let body = json!({ "model": model, "prompt": prompt, "stream": false });

        let res = self.client.post(url)
            .json(&body)
            .send().await.map_err(|e| e.to_string())?;

        let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        data["response"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Ollama no devolvió 'response'".into())
    }
}