use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// TODO Definir mejor, esta demasiado estricto, contradice la dinamicidad esperada
// TODO redefinir para que en lugar de guardarlo en una BD,
// se guarden los artifacts en temp_files dentro del mismo workspace

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

/// Resultado tangible y persistente de un proceso de pensamiento.
/// Versionado y validado mediante sumas de verificación.
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