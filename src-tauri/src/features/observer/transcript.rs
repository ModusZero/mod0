use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use serde_json::Value;

/// Cuántos mensajes recientes se recuerdan para asociar texto con herramientas.
const REMEMBERED_MESSAGES: usize = 64;

/// Lo nuevo que apareció en el transcript desde la última lectura.
#[derive(Debug, Default, PartialEq)]
pub struct TranscriptUpdate {
    /// `(tool_use_id, texto previo)`: lo que la IA dijo justo antes de llamar la herramienta.
    pub intents: Vec<(String, String)>,
    /// `tool_use_id` de herramientas que terminaron con error.
    pub errors: Vec<String>,
    pub context_tokens: Option<u64>,
    pub model: Option<String>,
}

/// Lector incremental del transcript JSONL que escribe Claude Code. Es de solo lectura y
/// no consume tokens: únicamente lee los bytes nuevos añadidos al archivo.
#[derive(Debug, Default)]
pub struct TranscriptCursor {
    offset: u64,
    texts: HashMap<String, String>,
    order: VecDeque<String>,
}

impl TranscriptCursor {
    pub fn poll(&mut self, path: &Path) -> TranscriptUpdate {
        let mut update = TranscriptUpdate::default();

        let Ok(mut file) = File::open(path) else {
            return update;
        };
        let length = file.metadata().map(|m| m.len()).unwrap_or(0);

        // Si el archivo se truncó o rotó, vuelve a empezar.
        if length < self.offset {
            self.offset = 0;
        }
        if length == self.offset || file.seek(SeekFrom::Start(self.offset)).is_err() {
            return update;
        }

        let mut bytes = Vec::new();
        if file.read_to_end(&mut bytes).is_err() {
            return update;
        }

        // Solo se consumen líneas completas; el resto se relee en la próxima lectura.
        let Some(last_newline) = bytes.iter().rposition(|b| *b == b'\n') else {
            return update;
        };
        self.offset += (last_newline + 1) as u64;

        for line in String::from_utf8_lossy(&bytes[..last_newline]).lines() {
            if let Ok(value) = serde_json::from_str::<Value>(line) {
                self.parse_line(&value, &mut update);
            }
        }
        update
    }

    fn parse_line(&mut self, line: &Value, update: &mut TranscriptUpdate) {
        // Los subagentes (sidechain) tienen su propio contexto; no mezclarlos.
        if line.get("isSidechain").and_then(Value::as_bool).unwrap_or(false) {
            return;
        }

        match line.get("type").and_then(Value::as_str) {
            Some("assistant") => self.parse_assistant(line, update),
            Some("user") => parse_user(line, update),
            _ => {}
        }
    }

    fn parse_assistant(&mut self, line: &Value, update: &mut TranscriptUpdate) {
        let Some(message) = line.get("message") else {
            return;
        };
        let message_id = message
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("-")
            .to_string();

        if let Some(model) = message.get("model").and_then(Value::as_str) {
            if !model.starts_with('<') {
                update.model = Some(model.to_string());
            }
        }

        if let Some(usage) = message.get("usage") {
            let read = |key: &str| usage.get(key).and_then(Value::as_u64).unwrap_or(0);
            let total =
                read("input_tokens") + read("cache_read_input_tokens") + read("cache_creation_input_tokens");
            if total > 0 {
                update.context_tokens = Some(total);
            }
        }

        let Some(blocks) = message.get("content").and_then(Value::as_array) else {
            return;
        };

        for block in blocks {
            match block.get("type").and_then(Value::as_str) {
                Some("text") => {
                    let text = block.get("text").and_then(Value::as_str).unwrap_or("").trim();
                    if !text.is_empty() {
                        self.remember(&message_id, text);
                    }
                }
                Some("tool_use") => {
                    let id = block.get("id").and_then(Value::as_str);
                    if let (Some(id), Some(text)) = (id, self.texts.get(&message_id)) {
                        update.intents.push((id.to_string(), text.clone()));
                    }
                }
                _ => {}
            }
        }
    }

    fn remember(&mut self, message_id: &str, text: &str) {
        if !self.texts.contains_key(message_id) {
            self.order.push_back(message_id.to_string());
            if self.order.len() > REMEMBERED_MESSAGES {
                if let Some(old) = self.order.pop_front() {
                    self.texts.remove(&old);
                }
            }
        }
        self.texts.insert(message_id.to_string(), text.to_string());
    }
}

fn parse_user(line: &Value, update: &mut TranscriptUpdate) {
    let Some(blocks) = line.pointer("/message/content").and_then(Value::as_array) else {
        return;
    };

    for block in blocks {
        let is_result = block.get("type").and_then(Value::as_str) == Some("tool_result");
        let is_error = block.get("is_error").and_then(Value::as_bool).unwrap_or(false);

        if is_result && is_error {
            if let Some(id) = block.get("tool_use_id").and_then(Value::as_str) {
                update.errors.push(id.to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn reads_intents_usage_and_errors_incrementally() {
        let dir = std::env::temp_dir().join(format!("mod0-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("t.jsonl");

        let mut file = File::create(&path).unwrap();
        writeln!(
            file,
            r#"{{"type":"assistant","message":{{"id":"m1","model":"claude-x","usage":{{"input_tokens":10,"cache_read_input_tokens":90,"cache_creation_input_tokens":5}},"content":[{{"type":"text","text":"Voy a leer el archivo"}}]}}}}"#
        )
        .unwrap();
        writeln!(
            file,
            r#"{{"type":"assistant","message":{{"id":"m1","content":[{{"type":"tool_use","id":"tool1","name":"Read","input":{{}}}}]}}}}"#
        )
        .unwrap();
        writeln!(
            file,
            r#"{{"type":"user","message":{{"content":[{{"type":"tool_result","tool_use_id":"tool1","is_error":true,"content":"x"}}]}}}}"#
        )
        .unwrap();
        write!(file, r#"{{"type":"assistant","message":{{"id":"m2""#).unwrap();
        file.flush().unwrap();

        let mut cursor = TranscriptCursor::default();
        let update = cursor.poll(&path);
        assert_eq!(
            update.intents,
            vec![("tool1".to_string(), "Voy a leer el archivo".to_string())]
        );
        assert_eq!(update.errors, vec!["tool1".to_string()]);
        assert_eq!(update.context_tokens, Some(105));
        assert_eq!(update.model.as_deref(), Some("claude-x"));

        // La línea incompleta se conserva hasta que llegue su salto de línea.
        assert_eq!(cursor.poll(&path), TranscriptUpdate::default());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn sidechain_lines_are_ignored() {
        let mut cursor = TranscriptCursor::default();
        let mut update = TranscriptUpdate::default();
        let line: Value = serde_json::from_str(
            r#"{"type":"assistant","isSidechain":true,"message":{"id":"x","usage":{"input_tokens":999}}}"#,
        )
        .unwrap();
        cursor.parse_line(&line, &mut update);
        assert_eq!(update, TranscriptUpdate::default());
    }
}
