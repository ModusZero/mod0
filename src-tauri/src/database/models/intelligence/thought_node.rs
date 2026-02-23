use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Clasifica los pasos del "Grafo de Pensamiento" del agente (decisiones tomadas, acciones ejecutadas u observaciones recibidas).
/// -`Observation`: Pensamiento del agente, normalmente en planeación o en edge cases.
/// -`Decision`: Bifurcación de opciones en las que el user debe decidir.
/// -`Action`: Ejecución de un task específico.
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum NodeType {
    Observation,
    Decision,
    Action,
}

/// Unidad atómica de razonamiento vinculada a una sesión, permitiendo la trazabilidad jerárquica de la lógica de la IA.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ThoughtNode {
    pub id: i64,
    pub session_id: String,
    pub parent_id: Option<i64>,
    pub node_type: NodeType,
    pub content: String,
    pub status: super::super::shared::status::Status,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
}