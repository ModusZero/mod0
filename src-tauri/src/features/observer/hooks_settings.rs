use std::path::Path;

use serde_json::{Value, json};

use crate::core::error::AppResult;
use crate::infrastructure::storage;

/// Eventos estables de Claude Code que MOD0 observa. Se evitan eventos recientes para que
/// versiones antiguas de la CLI no rechacen el archivo de configuración.
const OBSERVED_EVENTS: [&str; 9] = [
    "SessionStart",
    "SessionEnd",
    "UserPromptSubmit",
    "PreToolUse",
    "PostToolUse",
    "Notification",
    "Stop",
    "SubagentStop",
    "PreCompact",
];

/// Settings adicionales (`claude --settings <archivo>`) con hooks HTTP hacia MOD0.
/// Se combinan con los hooks propios del usuario en vez de reemplazarlos.
pub fn render(url: &str) -> Value {
    let handler = json!([{ "hooks": [{ "type": "http", "url": url, "timeout": 5 }] }]);
    let hooks: serde_json::Map<String, Value> = OBSERVED_EVENTS
        .iter()
        .map(|event| ((*event).to_string(), handler.clone()))
        .collect();

    json!({ "hooks": hooks })
}

pub fn write(path: &Path, url: &str) -> AppResult<()> {
    storage::write_json_atomic(path, &render(url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_every_observed_event_with_an_http_hook() {
        let settings = render("http://127.0.0.1:1/hook/a/b");
        let hooks = settings["hooks"].as_object().unwrap();
        assert_eq!(hooks.len(), OBSERVED_EVENTS.len());
        assert_eq!(hooks["Stop"][0]["hooks"][0]["type"], "http");
    }
}
