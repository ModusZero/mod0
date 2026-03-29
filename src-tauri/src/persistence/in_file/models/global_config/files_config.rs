use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilesConfig {
    /// Guardado automático de archivos a nivel sistema.
    pub auto_save: super::editor::AutoSaveStrategy,
    /// Retraso en milisegundos si auto_save es 'AfterDelay'.
    pub auto_save_delay: u64,
    
    /// Exclusiones globales de búsqueda y visualización.
    /// Ej: "**/.git": true, "**/node_modules": true
    pub exclude: std::collections::HashMap<String, bool>,
    
    /// Confirmaciones de UI
    pub explorer_confirm_delete: bool,
    pub explorer_confirm_drag_and_drop: bool,
}