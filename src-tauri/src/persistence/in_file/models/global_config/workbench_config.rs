use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StartupEditor {
    /// Abre un editor vacío.
    NewUntitledFile,
    /// Abre la página de bienvenida.
    WelcomePage,
    /// Abre el editor de "Release Notes".
    Readme,
    /// No abre nada.
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ControlsStyle {
    /// Estilo nativo del sistema operativo (Windows, macOS, Linux).
    Native,
    /// Estilo renderizado internamente por Mod0.
    Custom,
}

/// Configuración visual y estructural de la aplicación.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkbenchConfig {
    pub color_theme: String,
    pub icon_theme: String,
    pub startup_editor: StartupEditor,
    pub controls_style: ControlsStyle,
    
    /// Animaciones del IDE (reflejando tu extensión de animaciones)
    pub animations: AnimationsConfig,
    
    /// Asociaciones de editores para extensiones específicas
    /// Ej: "*.copilotmd" -> "vscode.markdown.preview.editor"
    pub editor_associations: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct AnimationsConfig {
    pub active: String, // ej: "Indent"
    pub command_palette: String, // ej: "Fade"
    pub cursor_animation: bool,
    pub focus_dimming_mode: String,
    pub scrolling: String,
    pub tabs: String,
}