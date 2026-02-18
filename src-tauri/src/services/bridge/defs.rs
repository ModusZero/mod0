use serde::{Deserialize, Serialize};
use crate::services::bridge::codec::CodecStrategy;

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

