use serde::{Deserialize, Serialize};
use crate::infrastructure::transport::codec::CodecStrategy;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CapabilityType { Lsp, Mcp }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeDefinition {
    pub id: String,
    pub capability: CapabilityType,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub strategy: CodecStrategy,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum ProviderType { Vcs, Ai }

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum AiDriver {
    Anthropic,
    Ollama,
    GoogleAI,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum VcsDriver {
    Github,
    Gitlab,
    GitLocal,
    GitBucket,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VcsCapabilities {
    pub can_pull_request: bool,
    pub is_cloud: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
pub enum GitAction {
    Commit,
    PullRequest,
    Other
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderConfig {
    pub id: String,
    pub r#type: ProviderType,
    pub ai_driver: Option<AiDriver>,
    pub vcs_driver: Option<VcsDriver>,
    pub token: String,
    pub endpoint: Option<String>,
    pub settings: serde_json::Value,
}

