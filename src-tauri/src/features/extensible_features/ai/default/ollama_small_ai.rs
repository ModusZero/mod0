// TODO Algo asi

// use async_trait::async_trait;
// use reqwest::Client;
// use serde_json::json;

// use async_trait::async_trait;
// use std::process::Command;

// use serde::{Deserialize, Serialize};
// use crate::infrastructure::transport::codec::CodecStrategy;

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
// pub enum CapabilityType { Lsp, Mcp }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct RuntimeDefinition {
//     pub id: String,
//     pub capability: CapabilityType,
//     pub name: String,
//     pub command: String,
//     pub args: Vec<String>,
//     pub strategy: CodecStrategy,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
// pub enum ProviderType { Vcs, Ai }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
// pub enum AiDriver {
//     Anthropic,
//     Ollama,
//     GoogleAI,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
// pub enum VcsDriver {
//     Github,
//     Gitlab,
//     GitLocal,
//     GitBucket,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct VcsCapabilities {
//     pub can_pull_request: bool,
//     pub is_cloud: bool,
// }

// #[derive(Debug, Serialize, Deserialize, sqlx::Type)]
// pub enum GitAction {
//     Commit,
//     PullRequest,
//     Other
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct ProviderConfig {
//     pub id: String,
//     pub r#type: ProviderType,
//     pub ai_driver: Option<AiDriver>,
//     pub vcs_driver: Option<VcsDriver>,
//     pub token: String,
//     pub endpoint: Option<String>,
//     pub settings: serde_json::Value,
// }


// /// Contrato fundamental para proveedores de Inteligencia Artificial.
// /// Define la estructura que cualquier driver debe implementar.
// #[async_trait]
// pub trait AiProvider: Send + Sync {
//     /// Crea una nueva instancia del driver con su configuración.
//     fn new(config: ProviderConfig) -> Self where Self: Sized;
//     /// Ejecuta una solicitud de completado de texto.
//     async fn completion(&self, prompt: &str) -> Result<String, String>;
// }

// pub struct OllamaProvider {
//     config: ProviderConfig,
//     client: Client,
// }

// #[async_trait]
// impl AiProvider for OllamaProvider {
//     fn new(config: ProviderConfig) -> Self {
//         Self { config, client: Client::new() }
//     }

//     async fn completion(&self, prompt: &str) -> Result<String, String> {
//         let url = self.config.endpoint.as_deref().unwrap_or("http://localhost:11434/api/generate");
//         let model = self.config.settings["model"].as_str().unwrap_or("llama3");

//         let body = json!({ "model": model, "prompt": prompt, "stream": false });

//         let res = self.client.post(url)
//             .json(&body)
//             .send().await.map_err(|e| e.to_string())?;

//         let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
//         data["response"].as_str()
//             .map(|s| s.to_string())
//             .ok_or_else(|| "Ollama no devolvió 'response'".into())
//     }
// }