use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use crate::services::bridge::{defs::ProviderConfig, providers::ai::AiProvider};

pub struct ClaudeProvider {
    config: ProviderConfig,
    client: Client,
}

impl ClaudeProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self { config, client: Client::new() }
    }
}

#[async_trait]
impl AiProvider for ClaudeProvider {
    async fn completion(&self, prompt: &str) -> Result<String, String> {
        let model = self.config.settings["model"].as_str().unwrap_or("claude-3-5-sonnet-20240620");
        
        let body = json!({
            "model": model,
            "max_tokens": 4096,
            "messages": [{"role": "user", "content": prompt}]
        });

        let res = self.client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.config.token)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send().await.map_err(|e| e.to_string())?;

        let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        data["content"][0]["text"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("Error en Claude: {:?}", data))
    }
}