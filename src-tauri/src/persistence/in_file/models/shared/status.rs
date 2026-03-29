use serde::{Deserialize, Serialize};

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