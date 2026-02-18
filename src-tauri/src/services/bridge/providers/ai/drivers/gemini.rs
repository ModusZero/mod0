use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use crate::services::bridge::{defs::ProviderConfig, providers::ai::AiProvider};

pub struct GeminiProvider {
    config: ProviderConfig,
    client: Client,
}

impl GeminiProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self { config, client: Client::new() }
    }
}

#[async_trait]
impl AiProvider for GeminiProvider {
    async fn completion(&self, prompt: &str) -> Result<String, String> {
        let model = self.config.settings["model"].as_str().unwrap_or("gemini-1.5-pro");
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, self.config.token
        );

        let body = json!({
            "contents": [{ "parts": [{ "text": prompt }] }]
        });

        let res = self.client.post(url)
            .json(&body)
            .send().await.map_err(|e| e.to_string())?;

        let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        data["candidates"][0]["content"]["parts"][0]["text"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("Error en Gemini: {:?}", data))
    }
}