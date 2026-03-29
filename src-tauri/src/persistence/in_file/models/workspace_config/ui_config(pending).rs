use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Serialize, Deserialize, Type)]
pub enum PanelSide {
    Left,
    Right,
    Bottom,
}

#[derive(Debug, FromRow)]
pub struct LayoutState {
    pub view_id: String,
    pub is_visible: bool,
    pub side: PanelSide,
    pub width: u32,
    /// Estado interno de la vista (ej: scroll position) en JSON.
    pub internal_state_json: String,
}