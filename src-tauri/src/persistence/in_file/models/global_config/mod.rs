pub mod ai_config;
pub mod editor_config;
pub mod extensions_registry;
pub mod files_config;
pub mod git_config;
pub mod security_config;
pub mod terminal_config;
pub mod workbench_config;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Estructura raíz que representa el archivo de configuración completo.
/// Este mismo modelo se usa para Global (`~/.mod0/settings.toml`) 
/// y Workspace (`[proyecto]/.mod0/settings.toml`).
/// En tiempo de ejecución, el Workspace hace un "merge" profundo sobre el Global.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mod0Config {
    pub editor: editor::EditorConfig,
    pub workbench: workbench::WorkbenchConfig,
    pub files: files::FilesConfig,
    pub terminal: terminal::TerminalConfig,
    pub git: git::GitConfig,
    pub security: security::SecurityConfig,
    pub ai_agents: ai_agents::AiAgentsConfig,
    
    /// Ajustes específicos por lenguaje (ej: "[typescriptreact]" -> EditorConfig parcial)
    #[serde(default)]
    pub language_overrides: HashMap<String, editor::EditorConfig>,

    /// Ajustes inyectados por extensiones (Extension Configuration Space).
    /// Ej: "python.defaultInterpreterPath": Value::String("...")
    #[serde(flatten)]
    pub extensions: HashMap<String, toml::Value>,
}