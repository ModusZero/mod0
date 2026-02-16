use serde::{Deserialize, Serialize};
use crate::services::bridge::codec::CodecStrategy;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CapabilityType { Lsp, Mcp, Vcs, Ai }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuntimeDefinition {
    pub id: String,
    pub capability: CapabilityType,
    pub name: String,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    pub strategy: CodecStrategy,
    pub extensions: Option<Vec<String>>,
    pub confidence: f32, // 0.0 a 1.0
}