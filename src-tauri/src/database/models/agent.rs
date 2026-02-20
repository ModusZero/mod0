use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Define el contexto de ejecución de una sesión.
/// - `UserChat`: Sesión iniciada por un user al abrir un chat nuevo.
/// - `AgentTask`: Sesión iniciada de forma autónoma por el agente para subdividir sus tasks.
/// - `Terminal`: Sesión de terminal iniciada por el user o por el agent.
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum SessionType {
    UserChat,
    AgentTask,
    Terminal,
}

/// Estado vital de una sesión o pensamiento, controlando flujos de aprobación y ciclos de vida.
/// -`Active`: Sesión en la que se está trabajando actualmente.
/// -`Merging`: Sesión casi terminada, solo falta subir los cambios al main codebase.
/// -`Completed`: Sesión completada con cambios guardados.
/// -`Aborted`: Sesión abortada por el user.
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Status {
    Active,
    Merging,
    Completed,
    Aborted,
}

/// Clasifica los pasos del "Grafo de Pensamiento" del agente (decisiones tomadas, acciones ejecutadas u observaciones recibidas).
/// -`Observation`: Pensamiento del agente, normalmente en planeación o en edge cases.
/// -`Decision`: Bifurcación de opciones en las que el user debe decidir.
/// -`Action`: Ejecución de un task específico.
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "PascalCase")]
pub enum NodeType {
    Observation,
    Decision,
    Action,
}

/// Clasificador de productos generados por la IA, permitiendo al frontend renderizar el slot adecuado (UI, Schemas, etc.).
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum ArtifactType {
    UiLayout,
    Schema,
    Scene3d,
    Flowchart,
    Documentation,
}

/// Contenedor principal de un hilo de trabajo; incluye soporte para multitasking en entornos controlados y "Ghost Sessions" destinadas a pruebas en sandboxes.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: String,
    pub project_path: String,
    pub session_type: SessionType,
    pub name: Option<String>,
    pub branch_name: Option<String>,
    pub worktree_path: Option<String>,
    pub status: Status,
    pub is_ghost_session: bool,
    pub base_commit_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Unidad atómica de razonamiento vinculada a una sesión, permitiendo la trazabilidad jerárquica de la lógica de la IA.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ThoughtNode {
    pub id: i64,
    pub session_id: String,
    pub parent_id: Option<i64>,
    pub node_type: NodeType,
    pub content: String,
    pub status: Status,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Resultado tangible y persistente de un proceso de pensamiento, versionado y validado mediante sumas de verificación.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Artifact {
    pub id: String,
    pub session_id: String,
    pub thought_node_id: Option<i64>,
    pub artifact_type: ArtifactType,
    pub content: String,
    pub version: i64,
    pub checksum: Option<String>,
    pub status: String,
}