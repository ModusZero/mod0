use serde::{Deserialize, Serialize};

/// Estado operativo de una sesión. Determina cuánta atención merece.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    /// Arrancando la CLI.
    Starting,
    /// La IA está trabajando.
    Working,
    /// Bloqueada esperando una decisión (permiso o pregunta). Requiere atención inmediata.
    Waiting,
    /// Terminó un turno con cambios: toca revisar.
    Review,
    /// Sin actividad ni nada pendiente.
    Idle,
    /// El proceso terminó.
    Exited,
}

/// Qué está haciendo la sesión ahora mismo.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NowInfo {
    /// Familia de la acción: read | write | exec | search | web | agent | plan | think | other.
    pub kind: String,
    pub label: String,
    pub target: Option<String>,
}

/// Indicadores que realmente merecen atención (ver `docs/CONTEXT.md`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SessionKpis {
    pub turns: u32,
    pub tool_calls: u32,
    pub errors: u32,
    pub files_changed: u32,
    pub additions: u32,
    pub deletions: u32,
    /// Tokens de contexto del último mensaje (entrada + caché).
    pub context_tokens: u64,
}

/// Un punto del plan que la IA mantiene con la herramienta `TodoWrite`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlanItem {
    pub content: String,
    /// `pending`, `in_progress` o `completed`.
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: String,
    pub name: String,
    pub project_path: String,
    pub cwd: String,
    pub branch: Option<String>,
    pub base_sha: Option<String>,
    pub worktree: bool,
    pub profile_id: String,
    pub profile_name: String,
    pub status: SessionStatus,
    pub status_since: i64,
    pub created_at: i64,
    pub now: Option<NowInfo>,
    pub kpis: SessionKpis,
    /// Plan actual de la IA (checklist). Vacío si la CLI no usa `TodoWrite`.
    #[serde(default)]
    pub plan: Vec<PlanItem>,
    pub model: Option<String>,
    pub last_message: Option<String>,
    pub claude_session_id: Option<String>,
    pub transcript_path: Option<String>,
    /// Archivos tocados por la sesión (rutas absolutas, máximo 200).
    pub touched: Vec<String>,
    pub exit_code: Option<i32>,
    pub reviewed: bool,
    pub color_idx: u8,
    /// Si la CLI reporta eventos (hooks); si no, el estado se infiere de la salida del PTY.
    pub observed: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionRequest {
    pub name: String,
    pub profile_id: String,
    pub project_path: String,
    #[serde(default)]
    pub worktree: bool,
    pub branch: Option<String>,
    pub prompt: Option<String>,
}
