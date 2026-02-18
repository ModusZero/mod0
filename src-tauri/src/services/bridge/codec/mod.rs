pub mod rpc_json;
pub mod line_stream;
pub mod raw_bytes;

use serde::{Deserialize, Serialize};

pub use crate::services::bridge::codec::rpc_json::JsonRpcMessage;
pub use crate::services::bridge::codec::line_stream::LineCodec;
pub use crate::services::bridge::codec::raw_bytes::RawCodec;

/// Representa cualquier dato que pueda viajar por el bridge.
pub enum BridgePayload {
    /// Para protocolos como LSP o MCP.
    JsonRpc(JsonRpcMessage),
    /// Para flujos de texto plano (logs, shell outputs).
    Line(String),
    /// Para datos binarios (VCS patches, imágenes, archivos).
    Raw(Vec<u8>),
}

/// Define cómo se debe empaquetar el payload para el transporte.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CodecStrategy {
    /// Formato: Content-Length: n\r\n\r\n{json}
    LspHeader,
    /// Formato: {data}\n
    NewLine,
    /// Formato: [bytes]
    Raw,
}

impl BridgePayload {
    /// Helper maestro: Transforma cualquier payload en bytes según la estrategia.
    /// Esto es lo que llamarán tus transports (Stdio, WS, HTTP).
    pub fn encode(self, strategy: CodecStrategy) -> Result<Vec<u8>, String> {
        match (self, strategy) {
            // Caso LSP / MCP
            (BridgePayload::JsonRpc(msg), CodecStrategy::LspHeader) => {
                let json = serde_json::to_string(&msg).map_err(|e| e.to_string())?;
                Ok(format!("Content-Length: {}\r\n\r\n{}", json.len(), json).into_bytes())
            },
            
            // Caso JSON-RPC por línea (algunos MCPs simples)
            (BridgePayload::JsonRpc(msg), CodecStrategy::NewLine) => {
                let json = serde_json::to_string(&msg).map_err(|e| e.to_string())?;
                Ok(LineCodec::encode(&json))
            },

            // Caso Línea de texto
            (BridgePayload::Line(text), CodecStrategy::NewLine) => {
                Ok(LineCodec::encode(&text))
            },

            // Caso Binario Puro
            (BridgePayload::Raw(bytes), CodecStrategy::Raw) => {
                Ok(RawCodec::encode(bytes))
            },

            // Fallbacks de seguridad
            (BridgePayload::Line(text), CodecStrategy::Raw) => Ok(text.into_bytes()),
            _ => Err("Combinación de Payload y Estrategia no válida".into()),
        }
    }
}