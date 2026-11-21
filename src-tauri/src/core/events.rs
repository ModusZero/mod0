use serde::Serialize;

use crate::features::observer::graph::ThoughtNode;

/// Nombres de los eventos que el backend emite hacia el frontend.
pub const SESSION_UPSERT: &str = "session:upsert";
pub const SESSION_REMOVED: &str = "session:removed";
pub const GRAPH_DELTA: &str = "graph:delta";
pub const FS_CHANGED: &str = "fs:changed";

/// Nodos nuevos o actualizados del thinking graph de una sesión.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDelta {
    pub session_id: String,
    pub nodes: Vec<ThoughtNode>,
}

/// Indica que algo cambió en el disco dentro de `cwd` (activity-driven, sin watchers).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FsChanged {
    pub cwd: String,
}
