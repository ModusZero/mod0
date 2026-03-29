use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// TODO redefinir para que en lugar de guardarlo en una BD,
// se guarde el ultimo estado en temp_files dentro del mismo workspace cada cierto tiempo

/// Gestiona la persistencia del estado visual por proyecto, incluyendo pestañas abiertas, el archivo activo y la configuración del layout de paneles.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EditorState {
    pub project_path: String,
    pub open_tabs: String,
    pub active_tab: Option<String>,
    pub layout_config: Option<String>,
}