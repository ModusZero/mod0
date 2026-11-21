use std::sync::Arc;
use std::thread;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

use crate::config::ConfigStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WebhookConfig {
    pub enabled: bool,
    /// Admite `{title}`, `{message}`, `{session}`, `{status}` y sus variantes `_url` (codificada).
    pub url: String,
    /// `GET` o `POST`.
    pub method: String,
    /// Cuerpo del POST. Admite también `{title_json}` y `{message_json}`.
    pub body: String,
    pub content_type: String,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            url: String::new(),
            method: "POST".into(),
            body: "{message}".into(),
            content_type: "text/plain; charset=utf-8".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NotificationConfig {
    pub desktop: bool,
    pub only_when_unfocused: bool,
    pub on_needs_you: bool,
    pub on_session_done: bool,
    pub on_all_idle: bool,
    pub webhook: WebhookConfig,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            desktop: true,
            only_when_unfocused: true,
            on_needs_you: true,
            on_session_done: false,
            on_all_idle: true,
            webhook: WebhookConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifyKind {
    NeedsYou,
    SessionDone,
    AllIdle,
    Test,
}

/// Envía avisos por escritorio y por webhook (ntfy, Telegram, WhatsApp vía CallMeBot, etc.).
#[derive(Clone)]
pub struct Notifier {
    app: AppHandle,
    config: Arc<ConfigStore>,
}

impl Notifier {
    pub fn new(app: AppHandle, config: Arc<ConfigStore>) -> Self {
        Self { app, config }
    }

    pub fn send(&self, kind: NotifyKind, title: &str, message: &str, session: &str, status: &str) {
        let config = self.config.get().notifications;
        let enabled = match kind {
            NotifyKind::NeedsYou => config.on_needs_you,
            NotifyKind::SessionDone => config.on_session_done,
            NotifyKind::AllIdle => config.on_all_idle,
            NotifyKind::Test => true,
        };
        if !enabled {
            return;
        }

        let focused = self
            .app
            .get_webview_window("main")
            .and_then(|window| window.is_focused().ok())
            .unwrap_or(false);

        if config.desktop && (kind == NotifyKind::Test || !(config.only_when_unfocused && focused)) {
            let _ = self
                .app
                .notification()
                .builder()
                .title(title)
                .body(message)
                .show();
        }

        if config.webhook.enabled && !config.webhook.url.trim().is_empty() {
            let vars = Vars {
                title: title.to_string(),
                message: message.to_string(),
                session: session.to_string(),
                status: status.to_string(),
            };
            let webhook = config.webhook.clone();
            thread::spawn(move || {
                if let Err(error) = deliver(&webhook, &vars) {
                    eprintln!("[mod0] webhook falló: {error}");
                }
            });
        }
    }
}

struct Vars {
    title: String,
    message: String,
    session: String,
    status: String,
}

fn deliver(webhook: &WebhookConfig, vars: &Vars) -> Result<(), String> {
    let url = fill(&webhook.url, vars);
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(10)))
        .build()
        .into();

    if webhook.method.eq_ignore_ascii_case("GET") {
        agent.get(&url).call().map_err(|e| e.to_string())?;
    } else {
        agent
            .post(&url)
            .content_type(webhook.content_type.as_str())
            .send(fill(&webhook.body, vars))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Sustituye los marcadores de una plantilla.
fn fill(template: &str, vars: &Vars) -> String {
    let pairs = [
        ("title", vars.title.as_str()),
        ("message", vars.message.as_str()),
        ("session", vars.session.as_str()),
        ("status", vars.status.as_str()),
    ];

    let mut out = template.to_string();
    for (key, value) in pairs {
        out = out
            .replace(&format!("{{{key}_url}}"), &percent_encode(value))
            .replace(&format!("{{{key}_json}}"), &json_escape(value))
            .replace(&format!("{{{key}}}"), value);
    }
    out
}

fn percent_encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(byte as char),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

fn json_escape(value: &str) -> String {
    let quoted = serde_json::Value::String(value.to_string()).to_string();
    quoted[1..quoted.len() - 1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_plain_url_and_json_variants() {
        let vars = Vars {
            title: "Listo".into(),
            message: "a \"b\" c".into(),
            session: "api".into(),
            status: "review".into(),
        };
        assert_eq!(
            fill("https://x/?t={message_url}", &vars),
            "https://x/?t=a%20%22b%22%20c"
        );
        assert_eq!(
            fill("{\"m\":\"{message_json}\"}", &vars),
            "{\"m\":\"a \\\"b\\\" c\"}"
        );
        assert_eq!(fill("{session}:{status}", &vars), "api:review");
    }
}
