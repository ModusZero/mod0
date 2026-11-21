use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Perfil de CLI: qué ejecutable lanzar en cada sesión y cómo.
///
/// MOD0 no reimplementa ningún proveedor de IA: cada perfil es un envoltorio de una CLI
/// existente. BYOK e IA local se resuelven con `env` (por ejemplo `ANTHROPIC_BASE_URL`).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct CliProfile {
    pub id: String,
    pub name: String,
    /// Ejecutable (vacío = solo una terminal).
    pub command: String,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    /// Inyecta hooks HTTP (solo Claude Code) para observar la sesión con precisión.
    pub hooks: bool,
    /// Argumentos para reanudar; `{session}` se reemplaza por el id de sesión de la CLI.
    pub resume_args: Vec<String>,
}

pub fn default_profiles() -> Vec<CliProfile> {
    vec![
        CliProfile {
            id: "claude".into(),
            name: "Claude Code".into(),
            command: "claude".into(),
            hooks: true,
            resume_args: vec!["--resume".into(), "{session}".into()],
            ..CliProfile::default()
        },
        CliProfile {
            id: "codex".into(),
            name: "Codex CLI".into(),
            command: "codex".into(),
            ..CliProfile::default()
        },
        CliProfile {
            id: "gemini".into(),
            name: "Gemini CLI".into(),
            command: "gemini".into(),
            ..CliProfile::default()
        },
        CliProfile {
            id: "shell".into(),
            name: "Terminal".into(),
            ..CliProfile::default()
        },
    ]
}
