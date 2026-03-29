// TODO Algo asi

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


// /// Contrato fundamental para sistemas de control de versiones.
// #[async_trait]
// pub trait VcsProvider: Send + Sync {
//     fn new(config: ProviderConfig) -> Self where Self: Sized;
    
//     /// Información sobre qué puede hacer este driver
//     fn capabilities(&self) -> VcsCapabilities;

//     /// Método de salud para verificar si el token/config es válido antes de usarlo
//     async fn check_health(&self) -> bool;

//     /// Ejecuta un comando git.
//     async fn execute(&self, args: &[&str], path: &str) -> Result<String, String>;

//     /// Crear un Pull Request (opcional, validado por capabilities)
//     async fn create_pr(&self, title: &str, head: &str, base: &str) -> Result<String, String>;
// }

// pub struct LocalGitProvider {
//     _config: ProviderConfig,
// }

// #[async_trait]
// impl VcsProvider for LocalGitProvider {
//     fn new(config: ProviderConfig) -> Self {
//         Self { _config: config }
//     }

//     fn capabilities(&self) -> VcsCapabilities {
//         VcsCapabilities {
//             can_pull_request: false,
//             is_cloud: false,
//         }
//     }

//     async fn execute(&self, args: &[&str], path: &str) -> Result<String, String> {
//         let output = Command::new("git")
//             .current_dir(path)
//             .args(args)
//             .output()
//             .map_err(|e| e.to_string())?;

//         if output.status.success() {
//             Ok(String::from_utf8_lossy(&output.stdout).to_string())
//         } else {
//             Err(String::from_utf8_lossy(&output.stderr).to_string())
//         }
//     }

//     async fn create_pr(&self, _: &str, _: &str, _: &str) -> Result<String, String> {
//         Err("Git Local no soporta Pull Requests.".into())
//     }

//     async fn check_health(&self) -> bool {
//         // Verifica si git está instalado
//         Command::new("git").arg("--version").status().is_ok()
//     }
// }