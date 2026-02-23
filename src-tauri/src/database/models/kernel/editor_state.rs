use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Gestiona la persistencia del estado visual por proyecto, incluyendo pestañas abiertas, el archivo activo y la configuración del layout de paneles.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EditorState {
    pub project_path: String,
    pub open_tabs: String,
    pub active_tab: Option<String>,
    pub layout_config: Option<String>,
}