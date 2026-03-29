use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TerminalShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    CommandPrompt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TerminalConfig {
    /// Shell por defecto a utilizar.
    pub default_shell: TerminalShell,
    /// Buffer de scrollback (líneas máximas).
    pub scrollback_limit: u32,
    /// Familia de fuente específica para terminal.
    pub font_family: String,
    /// Tamaño de fuente para terminal.
    pub font_size: u32,
    /// Habilita el renderizado por GPU (si está disponible).
    pub gpu_acceleration: bool,
    /// Copiar automáticamente al seleccionar con el mouse.
    pub copy_on_selection: bool,
}