use crate::infrastructure::utils::transport::ProtocolType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    
    /// Ruta al binario (relativa a la carpeta de la extensión)
    pub main: String,         

    pub activation_events: Vec<String>, 
    pub contributions: Contributions,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Contributions {
    pub commands: Vec<CommandContribution>,
    pub protocol: ProtocolType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandContribution {
    pub command: String,
    pub title: String,
}