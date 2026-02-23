pub mod ws;
pub mod tcp;
pub mod http;
pub mod stdio;
pub mod process;
pub mod codec;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolType {
    Stdio,
    Tcp,
    Http,
    WebSocket,
}

impl Default for ProtocolType {
    fn default() -> Self {
        ProtocolType::Stdio
    }
}