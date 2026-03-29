use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiAgentsConfig {
    /// Activa la telemetría y métricas de uso de IA internas.
    pub stats_enabled: bool,
    
    /// Habilita el Command Center de chat en la barra de título.
    pub chat_command_center_enabled: bool,
    
    /// Habilita la galería y el soporte del Model Context Protocol (MCP).
    pub mcp_gallery_enabled: bool,
    
    /// Proveedor por defecto (ej: "ollama", "anthropic").
    pub default_provider: String,
    
    /// Reglas globales inyectadas en todos los prompts.
    pub global_system_instructions: Option<String>,
}