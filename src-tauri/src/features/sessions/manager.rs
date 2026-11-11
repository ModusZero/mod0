use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;

use serde_json::Value;
use tauri::{AppHandle, Emitter};

use super::launcher::{self, LaunchInput};
use super::model::{CreateSessionRequest, NowInfo, Session, SessionKpis, SessionStatus};
use super::profile::CliProfile;
use crate::config::ConfigStore;
use crate::core::error::{AppError, AppResult};
use crate::core::events::{self, FsChanged, GraphDelta};
use crate::core::state::Paths;
use crate::features::git;
use crate::features::notifications::{Notifier, NotifyKind};
use crate::features::observer::graph::{GraphBuilder, ThoughtNode, describe_tool, truncate};
use crate::features::observer::hook_server::{self, HookInfo};
use crate::features::observer::hooks_settings;
use crate::features::observer::transcript::{TranscriptCursor, TranscriptUpdate};
use crate::features::terminal::pty::Pty;
use crate::infrastructure::process::now_ms;
use crate::infrastructure::storage;

/// Paleta de colores (índices) que se reparte entre sesiones para distinguirlas de un vistazo.
const COLOR_SLOTS: usize = 8;
/// Tiempo sin recibir hooks tras el cual se pasa a inferir el estado desde la salida del PTY.
const HOOKS_GRACE_MS: i64 = 20_000;
/// Silencio del PTY a partir del cual una CLI sin hooks se considera inactiva.
const QUIET_MS: i64 = 2_500;
const MAX_TOUCHED: usize = 200;

struct Runtime {
    session: Session,
    pty: Option<Arc<Pty>>,
    run: u64,
    graph: GraphBuilder,
    graph_loaded: bool,
    cursor: TranscriptCursor,
    turn_edits: u32,
    last_refresh: i64,
    last_fs_emit: i64,
}

impl Runtime {
    fn new(session: Session) -> Self {
        Self {
            session,
            pty: None,
            run: 0,
            graph: GraphBuilder::default(),
            graph_loaded: false,
            cursor: TranscriptCursor::default(),
            turn_edits: 0,
            last_refresh: 0,
            last_fs_emit: 0,
        }
    }

    /// Carga el grafo guardado la primera vez que se necesita.
    fn ensure_graph(&mut self, paths: &Paths) {
        if self.graph_loaded {
            return;
        }
        self.graph_loaded = true;
        let nodes: Option<Vec<ThoughtNode>> = storage::read_json(&paths.graph_file(&self.session.id))
            .ok()
            .flatten();
        if let Some(nodes) = nodes {
            self.graph = GraphBuilder::from_nodes(nodes);
        }
    }
}

#[derive(Default)]
struct Inner {
    sessions: HashMap<String, Runtime>,
    order: Vec<String>,
    was_active: bool,
}

pub struct SessionManager {
    app: AppHandle,
    paths: Paths,
    config: Arc<ConfigStore>,
    notifier: Notifier,
    hook: OnceLock<HookInfo>,
    inner: Mutex<Inner>,
}

impl SessionManager {
    pub fn new(app: AppHandle, paths: Paths, config: Arc<ConfigStore>) -> Arc<Self> {
        let notifier = Notifier::new(app.clone(), config.clone());
        Arc::new(Self {
            app,
            paths,
            config,
            notifier,
            hook: OnceLock::new(),
            inner: Mutex::default(),
        })
    }

    pub fn start_hook_server(self: &Arc<Self>) -> AppResult<()> {
        let info = hook_server::start(self.clone())?;
        let _ = self.hook.set(info);
        Ok(())
    }

    pub fn hook_port(&self) -> Option<u16> {
        self.hook.get().map(|info| info.port)
    }

    /// Recupera las sesiones guardadas. Sus procesos ya no existen, así que quedan como
    /// finalizadas y se pueden reanudar con la CLI.
    pub fn load_persisted(&self) {
        let saved: Vec<Session> = storage::read_json(&self.paths.sessions_file())
            .ok()
            .flatten()
            .unwrap_or_default();
        let mut inner = self.inner.lock().expect("sessions lock");

        for mut session in saved {
            session.status = SessionStatus::Exited;
            session.now = None;
            inner.order.push(session.id.clone());
            inner.sessions.insert(session.id.clone(), Runtime::new(session));
        }
    }

    /// Vigila las sesiones sin hooks y dispara el aviso "todo terminó".
    pub fn start_ticker(self: &Arc<Self>) {
        let manager = self.clone();
        thread::Builder::new()
            .name("mod0-ticker".into())
            .spawn(move || {
                loop {
                    thread::sleep(Duration::from_secs(1));
                    manager.tick();
                }
            })
            .expect("ticker thread");
    }

    pub fn list(&self) -> Vec<Session> {
        let inner = self.inner.lock().expect("sessions lock");
        inner
            .order
            .iter()
            .filter_map(|id| inner.sessions.get(id).map(|rt| rt.session.clone()))
            .collect()
    }

    pub fn session(&self, id: &str) -> AppResult<Session> {
        let inner = self.inner.lock().expect("sessions lock");
        inner
            .sessions
            .get(id)
            .map(|rt| rt.session.clone())
            .ok_or_else(|| AppError::SessionNotFound(id.into()))
    }

    pub fn pty(&self, id: &str) -> AppResult<Arc<Pty>> {
        let inner = self.inner.lock().expect("sessions lock");
        let runtime = inner
            .sessions
            .get(id)
            .ok_or_else(|| AppError::SessionNotFound(id.into()))?;
        runtime
            .pty
            .clone()
            .ok_or_else(|| AppError::msg("la sesión no está en ejecución"))
    }

    pub fn graph(&self, id: &str) -> AppResult<Vec<ThoughtNode>> {
        let mut inner = self.inner.lock().expect("sessions lock");
        let runtime = inner
            .sessions
            .get_mut(id)
            .ok_or_else(|| AppError::SessionNotFound(id.into()))?;
        runtime.ensure_graph(&self.paths);
        Ok(runtime.graph.nodes().to_vec())
    }

    pub fn create(self: &Arc<Self>, req: CreateSessionRequest) -> AppResult<Session> {
        let config = self.config.get();
        let profile = find_profile(&config.profiles, &req.profile_id)
            .ok_or_else(|| AppError::msg("perfil de CLI desconocido"))?;

        let project = PathBuf::from(&req.project_path);
        if !project.is_dir() {
            return Err(AppError::msg("la carpeta del proyecto no existe"));
        }

        let name = if req.name.trim().is_empty() {
            profile.name.clone()
        } else {
            req.name.trim().to_string()
        };
        let base_sha = git::head_sha(&project);

        let (cwd, branch) = if req.worktree {
            let slug = git::slugify(&name);
            let branch = req
                .branch
                .as_deref()
                .map(str::trim)
                .filter(|b| !b.is_empty())
                .map(String::from)
                .unwrap_or_else(|| format!("mod0/{slug}"));
            (git::worktree_add(&project, &slug, &branch)?, Some(branch))
        } else {
            (project.clone(), git::current_branch(&project))
        };

        let id = uuid::Uuid::new_v4().simple().to_string()[..12].to_string();
        let now = now_ms();
        let session = {
            let mut inner = self.inner.lock().expect("sessions lock");
            let session = Session {
                id: id.clone(),
                name,
                project_path: req.project_path.clone(),
                cwd: cwd.to_string_lossy().into_owned(),
                branch,
                base_sha,
                worktree: req.worktree,
                profile_id: profile.id.clone(),
                profile_name: profile.name.clone(),
                status: SessionStatus::Starting,
                status_since: now,
                created_at: now,
                now: None,
                kpis: SessionKpis::default(),
                model: None,
                last_message: None,
                claude_session_id: None,
                transcript_path: None,
                touched: Vec::new(),
                exit_code: None,
                reviewed: false,
                color_idx: (inner.order.len() % COLOR_SLOTS) as u8,
                observed: false,
            };
            inner.order.push(id.clone());
            inner.sessions.insert(id.clone(), Runtime::new(session.clone()));
            session
        };

        if let Err(error) = self.spawn_pty(&id, false, req.prompt.as_deref()) {
            self.inner.lock().expect("sessions lock").sessions.remove(&id);
            self.inner
                .lock()
                .expect("sessions lock")
                .order
                .retain(|existing| existing != &id);
            if req.worktree {
                let _ = git::worktree_remove(&project, &cwd);
            }
            return Err(error);
        }

        let created = self.session(&id).unwrap_or(session);
        self.emit_session(&created);
        self.persist();
        Ok(created)
    }

    /// Reinicia una sesión finalizada, reanudando la conversación si la CLI lo permite.
    pub fn restart(self: &Arc<Self>, id: &str) -> AppResult<Session> {
        if self.pty(id).is_ok() {
            return Err(AppError::msg("la sesión sigue en ejecución"));
        }
        self.spawn_pty(id, true, None)?;
        let session = self.session(id)?;
        self.emit_session(&session);
        self.persist();
        Ok(session)
    }

    pub fn stop(&self, id: &str) -> AppResult<()> {
        self.pty(id)?.kill();
        Ok(())
    }

    pub fn remove(&self, id: &str, remove_worktree: bool) -> AppResult<()> {
        let runtime = {
            let mut inner = self.inner.lock().expect("sessions lock");
            inner.order.retain(|existing| existing != id);
            inner
                .sessions
                .remove(id)
                .ok_or_else(|| AppError::SessionNotFound(id.into()))?
        };

        if let Some(pty) = &runtime.pty {
            pty.kill();
        }
        let _ = std::fs::remove_file(self.paths.graph_file(id));
        let _ = std::fs::remove_file(self.paths.hooks_file(id));

        if remove_worktree && runtime.session.worktree {
            git::worktree_remove(
                Path::new(&runtime.session.project_path),
                Path::new(&runtime.session.cwd),
            )?;
        }

        let _ = self.app.emit(events::SESSION_REMOVED, id);
        self.persist();
        Ok(())
    }

    pub fn rename(&self, id: &str, name: &str) -> AppResult<Session> {
        let session = self.mutate(id, |rt| {
            rt.session.name = name.trim().to_string();
        })?;
        self.persist();
        Ok(session)
    }

    pub fn set_reviewed(&self, id: &str, reviewed: bool) -> AppResult<Session> {
        let session = self.mutate(id, |rt| {
            rt.session.reviewed = reviewed;
            if reviewed && rt.session.status == SessionStatus::Review {
                set_status(&mut rt.session, SessionStatus::Idle);
            }
        })?;
        self.persist();
        Ok(session)
    }

    pub fn shutdown(&self) {
        let inner = self.inner.lock().expect("sessions lock");
        for runtime in inner.sessions.values() {
            if let Some(pty) = &runtime.pty {
                pty.kill();
            }
        }
    }

    /// Procesa un evento de hook de Claude Code (ver `reduce`).
    pub fn on_hook(self: &Arc<Self>, id: &str, payload: Value) {
        let outcome = {
            let mut inner = self.inner.lock().expect("sessions lock");
            let Some(runtime) = inner.sessions.get_mut(id) else {
                return;
            };
            runtime.ensure_graph(&self.paths);
            reduce(runtime, &payload, now_ms())
        };

        if let Some(session) = &outcome.session {
            self.emit_session(session);
        }
        if !outcome.nodes.is_empty() {
            let _ = self.app.emit(
                events::GRAPH_DELTA,
                GraphDelta {
                    session_id: id.into(),
                    nodes: outcome.nodes,
                },
            );
        }
        if let Some(cwd) = outcome.fs_changed {
            let _ = self.app.emit(events::FS_CHANGED, FsChanged { cwd });
        }
        if outcome.save_graph {
            self.save_graph(id);
        }
        if outcome.persist {
            self.persist();
        }
        if outcome.refresh {
            self.refresh_changes(id);
        }
        if let Some((kind, title, message)) = outcome.notify {
            let name = outcome
                .session
                .as_ref()
                .map(|s| s.name.clone())
                .unwrap_or_default();
            let status = outcome
                .session
                .as_ref()
                .map(|s| format!("{:?}", s.status).to_lowercase())
                .unwrap_or_default();
            self.notifier.send(kind, &title, &message, &name, &status);
        }
    }

    /// Recalcula archivos cambiados y líneas +/- respecto a la base de la sesión.
    pub fn refresh_changes(self: &Arc<Self>, id: &str) {
        let manager = self.clone();
        let id = id.to_string();

        thread::spawn(move || {
            let Ok(session) = manager.session(&id) else {
                return;
            };
            let Ok(files) = git::changes(Path::new(&session.cwd), session.base_sha.as_deref()) else {
                return;
            };

            let updated = manager.mutate(&id, |rt| {
                rt.session.kpis.files_changed = files.len() as u32;
                rt.session.kpis.additions = files.iter().map(|f| f.additions).sum();
                rt.session.kpis.deletions = files.iter().map(|f| f.deletions).sum();
                if rt.session.status == SessionStatus::Review && files.is_empty() {
                    set_status(&mut rt.session, SessionStatus::Idle);
                }
            });
            if let Ok(session) = updated {
                manager.emit_session(&session);
            }
        });
    }

    fn spawn_pty(self: &Arc<Self>, id: &str, resume: bool, prompt: Option<&str>) -> AppResult<()> {
        let config = self.config.get();
        let session = self.session(id)?;
        let profile = find_profile(&config.profiles, &session.profile_id).unwrap_or_else(shell_profile);

        let (hook_url, settings_file) = match (profile.hooks, self.hook.get()) {
            (true, Some(info)) => {
                let url = info.url_for(id);
                let file = self.paths.hooks_file(id);
                hooks_settings::write(&file, &url)?;
                (Some(url), Some(file))
            }
            _ => (None, None),
        };

        let observed = settings_file.is_some();
        let plan = launcher::plan(LaunchInput {
            session: &session,
            profile: &profile,
            config: &config,
            hook_url,
            settings_file,
            resume,
            prompt,
        })?;

        let run = self.mutate_run(id)?;
        let (manager, session_id) = (self.clone(), id.to_string());
        let pty = Pty::spawn(plan.spec, move |code| manager.on_pty_exit(&session_id, run, code))?;
        if !plan.input.is_empty() {
            pty.write(plan.input.as_bytes())?;
        }

        self.mutate(id, |rt| {
            rt.pty = Some(Arc::new(pty));
            rt.session.observed = observed;
            rt.session.exit_code = None;
            rt.session.now = None;
            set_status(&mut rt.session, SessionStatus::Starting);
        })?;
        Ok(())
    }

    fn mutate_run(&self, id: &str) -> AppResult<u64> {
        let mut inner = self.inner.lock().expect("sessions lock");
        let runtime = inner
            .sessions
            .get_mut(id)
            .ok_or_else(|| AppError::SessionNotFound(id.into()))?;
        runtime.run += 1;
        Ok(runtime.run)
    }

    fn on_pty_exit(&self, id: &str, run: u64, code: Option<i32>) {
        let updated = {
            let mut inner = self.inner.lock().expect("sessions lock");
            let Some(runtime) = inner.sessions.get_mut(id) else {
                return;
            };
            // Ignora la salida tardía de una ejecución anterior (tras reiniciar).
            if runtime.run != run {
                return;
            }
            runtime.pty = None;
            runtime.session.exit_code = code;
            runtime.session.now = None;
            set_status(&mut runtime.session, SessionStatus::Exited);
            runtime.session.clone()
        };
        self.emit_session(&updated);
        self.persist();
    }

    /// Aplica un cambio a una sesión y devuelve su estado resultante.
    fn mutate(&self, id: &str, change: impl FnOnce(&mut Runtime)) -> AppResult<Session> {
        let mut inner = self.inner.lock().expect("sessions lock");
        let runtime = inner
            .sessions
            .get_mut(id)
            .ok_or_else(|| AppError::SessionNotFound(id.into()))?;
        change(runtime);
        Ok(runtime.session.clone())
    }

    fn tick(&self) {
        let config = self.config.get();
        let plain_shell: Vec<String> = config
            .profiles
            .iter()
            .filter(|p| p.command.trim().is_empty())
            .map(|p| p.id.clone())
            .collect();
        let now = now_ms();

        let mut changed: Vec<Session> = Vec::new();
        let mut digest: Option<(usize, usize)> = None;

        {
            let mut inner = self.inner.lock().expect("sessions lock");
            for runtime in inner.sessions.values_mut() {
                let Some(pty) = runtime.pty.clone() else {
                    continue;
                };
                let session = &mut runtime.session;

                // Los hooks nunca llegaron (versión antigua de la CLI): se infiere desde la salida.
                if session.observed
                    && session.status == SessionStatus::Starting
                    && now - session.status_since > HOOKS_GRACE_MS
                {
                    session.observed = false;
                }
                if session.observed
                    || matches!(
                        session.status,
                        SessionStatus::Waiting | SessionStatus::Review | SessionStatus::Exited
                    )
                {
                    continue;
                }

                let target =
                    if plain_shell.contains(&session.profile_id) || now - pty.last_output_ms() > QUIET_MS {
                        SessionStatus::Idle
                    } else {
                        SessionStatus::Working
                    };
                if session.status != target {
                    set_status(session, target);
                    changed.push(session.clone());
                }
            }

            let active = inner
                .sessions
                .values()
                .any(|rt| matches!(rt.session.status, SessionStatus::Working | SessionStatus::Waiting));
            if inner.was_active && !active {
                let review = inner
                    .sessions
                    .values()
                    .filter(|rt| rt.session.status == SessionStatus::Review)
                    .count();
                digest = Some((inner.sessions.len(), review));
            }
            inner.was_active = active;
        }

        for session in &changed {
            self.emit_session(session);
        }
        if let Some((total, review)) = digest {
            let message = format!("{total} sesiones en reposo, {review} por revisar");
            self.notifier
                .send(NotifyKind::AllIdle, "MOD0: todo terminó", &message, "", "idle");
        }
    }

    fn emit_session(&self, session: &Session) {
        let _ = self.app.emit(events::SESSION_UPSERT, session);
    }

    fn persist(&self) {
        if let Err(error) = storage::write_json_atomic(&self.paths.sessions_file(), &self.list()) {
            eprintln!("[mod0] no se pudieron guardar las sesiones: {error}");
        }
    }

    fn save_graph(&self, id: &str) {
        if let Ok(nodes) = self.graph(id) {
            let _ = storage::write_json_atomic(&self.paths.graph_file(id), &nodes);
        }
    }
}

fn find_profile(profiles: &[CliProfile], id: &str) -> Option<CliProfile> {
    profiles.iter().find(|profile| profile.id == id).cloned()
}

fn shell_profile() -> CliProfile {
    CliProfile {
        id: "shell".into(),
        name: "Terminal".into(),
        ..CliProfile::default()
    }
}

fn set_status(session: &mut Session, status: SessionStatus) {
    if session.status != status {
        session.status = status;
        session.status_since = now_ms();
    }
}

#[derive(Default)]
struct Outcome {
    session: Option<Session>,
    nodes: Vec<ThoughtNode>,
    notify: Option<(NotifyKind, String, String)>,
    fs_changed: Option<String>,
    save_graph: bool,
    persist: bool,
    refresh: bool,
}

fn is_write_tool(tool: &str) -> bool {
    matches!(tool, "Edit" | "MultiEdit" | "Write" | "NotebookEdit")
}

fn apply_transcript(
    update: TranscriptUpdate,
    runtime_graph: &mut GraphBuilder,
    session: &mut Session,
    out: &mut Outcome,
) {
    for (tool_use_id, text) in update.intents {
        out.nodes.extend(runtime_graph.set_intent(&tool_use_id, &text));
    }
    for tool_use_id in update.errors {
        if let Some(node) = runtime_graph.mark_error(&tool_use_id) {
            session.kpis.errors += 1;
            out.nodes.push(node);
        }
    }
    if let Some(tokens) = update.context_tokens {
        session.kpis.context_tokens = tokens;
    }
    if let Some(model) = update.model {
        session.model = Some(model);
    }
}

/// Reductor de eventos: transforma un hook de Claude Code en cambios de estado, nodos del
/// thinking graph y avisos. Es una función pura sobre `Runtime` para poder probarla.
fn reduce(runtime: &mut Runtime, payload: &Value, now: i64) -> Outcome {
    let Runtime {
        session,
        graph,
        cursor,
        turn_edits,
        last_refresh,
        last_fs_emit,
        ..
    } = runtime;
    let mut out = Outcome::default();

    let event = payload
        .get("hook_event_name")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let text = |key: &str| payload.get(key).and_then(Value::as_str);
    let tool = text("tool_name").unwrap_or_default();
    let tool_input = payload.get("tool_input").cloned().unwrap_or(Value::Null);
    let tool_use_id = text("tool_use_id");

    if let Some(cli_session) = text("session_id") {
        session.claude_session_id = Some(cli_session.to_string());
    }
    if let Some(path) = text("transcript_path") {
        session.transcript_path = Some(path.to_string());
    }
    session.observed = true;

    let mut poll = false;

    match event {
        "SessionStart" => {
            if matches!(session.status, SessionStatus::Starting | SessionStatus::Exited) {
                set_status(session, SessionStatus::Idle);
            }
        }
        "UserPromptSubmit" => {
            let prompt = text("prompt").or_else(|| text("user_input")).unwrap_or_default();
            session.kpis.turns += 1;
            session.reviewed = false;
            *turn_edits = 0;
            out.nodes.push(graph.begin_turn(prompt, now));
            session.now = Some(NowInfo {
                kind: "think".into(),
                label: "Pensando…".into(),
                target: None,
            });
            set_status(session, SessionStatus::Working);
            out.persist = true;
        }
        "PreToolUse" => {
            let info = describe_tool(tool, &tool_input);
            out.nodes
                .push(graph.tool_start(tool_use_id, tool, &tool_input, now));
            session.now = Some(NowInfo {
                kind: info.family.into(),
                label: info.label,
                target: info.target.clone(),
            });
            set_status(session, SessionStatus::Working);

            if is_write_tool(tool) {
                if let Some(target) = info.target {
                    if !session.touched.contains(&target) {
                        session.touched.push(target);
                        if session.touched.len() > MAX_TOUCHED {
                            session.touched.remove(0);
                        }
                    }
                }
            }
            poll = true;
        }
        "PostToolUse" => {
            let finished = graph.tool_end(tool_use_id, now).unwrap_or_else(|| {
                let started = graph.tool_start(tool_use_id, tool, &tool_input, now);
                graph.tool_end(Some(&started.id), now).unwrap_or(started)
            });
            out.nodes.push(finished);
            session.kpis.tool_calls += 1;
            session.now = Some(NowInfo {
                kind: "think".into(),
                label: "Pensando…".into(),
                target: None,
            });
            set_status(session, SessionStatus::Working);

            if is_write_tool(tool) {
                *turn_edits += 1;
            }
            if (is_write_tool(tool) || tool == "Bash") && now - *last_fs_emit > 800 {
                *last_fs_emit = now;
                out.fs_changed = Some(session.cwd.clone());
            }
            if is_write_tool(tool) && now - *last_refresh > 3_000 {
                *last_refresh = now;
                out.refresh = true;
            }
            poll = true;
        }
        "Notification" => {
            let message = text("message").unwrap_or("Claude necesita tu atención");
            let kind = text("notification_type").unwrap_or_default();
            let blocking = matches!(kind, "permission_prompt" | "elicitation_dialog")
                || (kind.is_empty() && message.to_lowercase().contains("permission"));

            if blocking {
                out.nodes.push(graph.notice(message, now));
                set_status(session, SessionStatus::Waiting);
                out.notify = Some((
                    NotifyKind::NeedsYou,
                    format!("{}: te necesita", session.name),
                    truncate(message, 140),
                ));
            }
        }
        "Stop" => {
            let message = text("last_assistant_message");
            out.nodes.push(graph.stop(message, now));
            session.now = None;
            session.last_message = message.map(|m| truncate(m, 400));
            let next = if *turn_edits > 0 {
                SessionStatus::Review
            } else {
                SessionStatus::Idle
            };
            set_status(session, next);
            out.notify = Some((
                NotifyKind::SessionDone,
                format!("{}: listo", session.name),
                truncate(message.unwrap_or("Terminó el turno"), 140),
            ));
            out.save_graph = true;
            out.persist = true;
            out.refresh = true;
            out.fs_changed = Some(session.cwd.clone());
            poll = true;
        }
        "PreCompact" => out.nodes.push(graph.notice("Compactando contexto", now)),
        "SessionEnd" => {
            session.now = None;
            set_status(session, SessionStatus::Idle);
        }
        _ => {}
    }

    if poll {
        if let Some(path) = session.transcript_path.clone() {
            let update = cursor.poll(Path::new(&path));
            apply_transcript(update, graph, session, &mut out);
        }
    }

    out.session = Some(session.clone());
    out
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn runtime() -> Runtime {
        Runtime::new(Session {
            id: "s1".into(),
            name: "api".into(),
            project_path: "/p".into(),
            cwd: "/p".into(),
            branch: None,
            base_sha: None,
            worktree: false,
            profile_id: "claude".into(),
            profile_name: "Claude Code".into(),
            status: SessionStatus::Starting,
            status_since: 0,
            created_at: 0,
            now: None,
            kpis: SessionKpis::default(),
            model: None,
            last_message: None,
            claude_session_id: None,
            transcript_path: None,
            touched: Vec::new(),
            exit_code: None,
            reviewed: false,
            color_idx: 0,
            observed: false,
        })
    }

    #[test]
    fn a_full_turn_goes_working_waiting_review() {
        let mut rt = runtime();

        reduce(
            &mut rt,
            &json!({ "hook_event_name": "SessionStart", "session_id": "abc" }),
            1,
        );
        assert_eq!(rt.session.status, SessionStatus::Idle);
        assert_eq!(rt.session.claude_session_id.as_deref(), Some("abc"));

        reduce(
            &mut rt,
            &json!({ "hook_event_name": "UserPromptSubmit", "prompt": "arregla" }),
            2,
        );
        assert_eq!(rt.session.status, SessionStatus::Working);
        assert_eq!(rt.session.kpis.turns, 1);

        let edit =
            json!({ "tool_name": "Edit", "tool_use_id": "t1", "tool_input": { "file_path": "/p/a.ts" } });
        let mut pre = edit.clone();
        pre["hook_event_name"] = json!("PreToolUse");
        let out = reduce(&mut rt, &pre, 3);
        assert_eq!(
            out.session.unwrap().now.unwrap().target.as_deref(),
            Some("/p/a.ts")
        );
        assert_eq!(rt.session.touched, vec!["/p/a.ts".to_string()]);

        let out = reduce(
            &mut rt,
            &json!({ "hook_event_name": "Notification", "notification_type": "permission_prompt", "message": "Permiso para Bash" }),
            4,
        );
        assert_eq!(rt.session.status, SessionStatus::Waiting);
        assert!(matches!(out.notify, Some((NotifyKind::NeedsYou, _, _))));

        let mut post = edit;
        post["hook_event_name"] = json!("PostToolUse");
        reduce(&mut rt, &post, 5);
        assert_eq!(rt.session.status, SessionStatus::Working);
        assert_eq!(rt.session.kpis.tool_calls, 1);

        let out = reduce(
            &mut rt,
            &json!({ "hook_event_name": "Stop", "last_assistant_message": "Listo" }),
            6,
        );
        assert_eq!(rt.session.status, SessionStatus::Review);
        assert!(out.refresh && out.save_graph);
        assert_eq!(rt.graph.nodes().len(), 4);
    }

    #[test]
    fn a_turn_without_edits_ends_idle_and_idle_prompt_is_ignored() {
        let mut rt = runtime();
        reduce(
            &mut rt,
            &json!({ "hook_event_name": "UserPromptSubmit", "prompt": "explica" }),
            1,
        );
        reduce(&mut rt, &json!({ "hook_event_name": "Stop" }), 2);
        assert_eq!(rt.session.status, SessionStatus::Idle);

        reduce(
            &mut rt,
            &json!({ "hook_event_name": "Notification", "notification_type": "idle_prompt" }),
            3,
        );
        assert_eq!(rt.session.status, SessionStatus::Idle);
    }
}
