use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Máximo de nodos que se conservan por sesión (se recortan los turnos más antiguos).
const MAX_NODES: usize = 1200;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    Prompt,
    Tool,
    Notice,
    Stop,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NodeStatus {
    Running,
    Ok,
    Error,
}

/// Un paso del razonamiento de la IA: el prompt, una herramienta usada, un aviso o el cierre del turno.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThoughtNode {
    pub id: String,
    pub turn: u32,
    pub seq: u32,
    pub kind: NodeKind,
    /// read | write | exec | search | web | agent | plan | think | other
    pub family: String,
    pub tool: Option<String>,
    pub label: String,
    pub detail: Option<String>,
    /// Lo que la IA dijo antes de usar la herramienta (el "por qué").
    pub intent: Option<String>,
    pub target: Option<String>,
    pub status: NodeStatus,
    pub parent: Option<String>,
    pub ts: i64,
    pub duration_ms: Option<i64>,
}

/// Descripción legible de una llamada a herramienta.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolInfo {
    pub family: &'static str,
    pub label: String,
    pub target: Option<String>,
    pub detail: Option<String>,
}

pub fn truncate(text: &str, max: usize) -> String {
    let clean = text.trim();
    if clean.chars().count() <= max {
        return clean.to_string();
    }
    let mut cut: String = clean.chars().take(max.saturating_sub(1)).collect();
    cut.push('…');
    cut
}

pub fn basename(path: &str) -> &str {
    path.rsplit(['/', '\\'])
        .find(|part| !part.is_empty())
        .unwrap_or(path)
}

fn str_field<'a>(input: &'a Value, keys: &[&str]) -> Option<&'a str> {
    keys.iter()
        .find_map(|key| input.get(*key).and_then(Value::as_str))
}

/// Traduce una llamada a herramienta de la CLI en una etiqueta corta + el archivo objetivo.
pub fn describe_tool(tool: &str, input: &Value) -> ToolInfo {
    let file = str_field(input, &["file_path", "notebook_path", "path"]);

    match tool {
        "Read" | "NotebookRead" => ToolInfo {
            family: "read",
            label: format!("Leer {}", file.map(basename).unwrap_or("archivo")),
            target: file.map(String::from),
            detail: None,
        },
        "Edit" | "MultiEdit" | "Write" | "NotebookEdit" => ToolInfo {
            family: "write",
            label: format!(
                "{} {}",
                if tool == "Write" { "Crear" } else { "Editar" },
                file.map(basename).unwrap_or("archivo")
            ),
            target: file.map(String::from),
            detail: None,
        },
        "Bash" | "BashOutput" | "KillShell" => {
            let command = str_field(input, &["command"]).unwrap_or(tool);
            ToolInfo {
                family: "exec",
                label: truncate(command.lines().next().unwrap_or(command), 70),
                target: None,
                detail: Some(truncate(command, 2000)),
            }
        }
        "Grep" | "Glob" => {
            let pattern = str_field(input, &["pattern"]).unwrap_or("");
            ToolInfo {
                family: "search",
                label: format!("{tool} {}", truncate(pattern, 50)),
                target: str_field(input, &["path"]).map(String::from),
                detail: Some(pattern.to_string()),
            }
        }
        "WebFetch" | "WebSearch" => {
            let what = str_field(input, &["url", "query"]).unwrap_or("");
            ToolInfo {
                family: "web",
                label: truncate(what, 70),
                target: None,
                detail: Some(what.to_string()),
            }
        }
        "Task" | "Agent" => {
            let description = str_field(input, &["description", "prompt"]).unwrap_or("subagente");
            ToolInfo {
                family: "agent",
                label: format!("Subagente: {}", truncate(description, 50)),
                target: None,
                detail: str_field(input, &["prompt"]).map(|p| truncate(p, 2000)),
            }
        }
        "TodoWrite" | "ExitPlanMode" => ToolInfo {
            family: "plan",
            label: "Plan".to_string(),
            target: None,
            detail: None,
        },
        other => ToolInfo {
            family: "other",
            label: other.trim_start_matches("mcp__").replace("__", " · "),
            target: file.map(String::from),
            detail: None,
        },
    }
}

/// Construye el thinking graph de una sesión a partir de los eventos de la CLI.
///
/// Regla de aristas: cada nodo nuevo cuelga de la última "cola completada". Las llamadas
/// paralelas comparten padre, así que se dibujan como ramas hermanas.
#[derive(Debug, Default)]
pub struct GraphBuilder {
    nodes: Vec<ThoughtNode>,
    turn: u32,
    seq: u32,
    tail: Option<String>,
    pending_intents: HashMap<String, String>,
}

impl GraphBuilder {
    pub fn from_nodes(nodes: Vec<ThoughtNode>) -> Self {
        let turn = nodes.iter().map(|n| n.turn).max().unwrap_or(0);
        let seq = nodes.iter().map(|n| n.seq).max().unwrap_or(0);
        let tail = nodes.last().map(|n| n.id.clone());
        Self {
            nodes,
            turn,
            seq,
            tail,
            pending_intents: HashMap::new(),
        }
    }

    pub fn nodes(&self) -> &[ThoughtNode] {
        &self.nodes
    }

    pub fn begin_turn(&mut self, prompt: &str, ts: i64) -> ThoughtNode {
        self.turn += 1;
        self.tail = None;
        let node = self.node(format!("p{}", self.turn), NodeKind::Prompt, "think", "Prompt", ts);
        let node = ThoughtNode {
            detail: Some(truncate(prompt, 2000)),
            label: truncate(prompt, 80),
            ..node
        };
        self.push(node, true)
    }

    pub fn tool_start(
        &mut self,
        tool_use_id: Option<&str>,
        tool: &str,
        input: &Value,
        ts: i64,
    ) -> ThoughtNode {
        let info = describe_tool(tool, input);
        let id = tool_use_id
            .map(String::from)
            .unwrap_or_else(|| format!("t{}", self.seq + 1));
        let intent = self.pending_intents.remove(&id);

        let node = ThoughtNode {
            tool: Some(tool.to_string()),
            target: info.target,
            detail: info.detail,
            intent,
            status: NodeStatus::Running,
            ..self.node(id, NodeKind::Tool, info.family, &info.label, ts)
        };
        self.push(node, false)
    }

    /// Marca la herramienta como terminada. Devuelve `None` si el nodo no existe.
    pub fn tool_end(&mut self, tool_use_id: Option<&str>, ts: i64) -> Option<ThoughtNode> {
        let index = match tool_use_id {
            Some(id) => self.nodes.iter().rposition(|n| n.id == id),
            None => self
                .nodes
                .iter()
                .rposition(|n| n.kind == NodeKind::Tool && n.status == NodeStatus::Running),
        }?;

        let node = &mut self.nodes[index];
        if node.status == NodeStatus::Running {
            node.status = NodeStatus::Ok;
            node.duration_ms = Some((ts - node.ts).max(0));
        }
        self.tail = Some(node.id.clone());
        Some(node.clone())
    }

    /// Marca una herramienta como fallida (se descubre al leer el transcript).
    pub fn mark_error(&mut self, tool_use_id: &str) -> Option<ThoughtNode> {
        let node = self.nodes.iter_mut().rev().find(|n| n.id == tool_use_id)?;
        if node.status == NodeStatus::Error {
            return None;
        }
        node.status = NodeStatus::Error;
        Some(node.clone())
    }

    /// Asocia el "por qué" a una herramienta. Si el nodo aún no existe, se guarda para después.
    pub fn set_intent(&mut self, tool_use_id: &str, text: &str) -> Option<ThoughtNode> {
        let text = truncate(text, 600);
        match self.nodes.iter_mut().rev().find(|n| n.id == tool_use_id) {
            Some(node) if node.intent.as_deref() != Some(text.as_str()) => {
                node.intent = Some(text);
                Some(node.clone())
            }
            Some(_) => None,
            None => {
                self.pending_intents.insert(tool_use_id.to_string(), text);
                None
            }
        }
    }

    pub fn notice(&mut self, text: &str, ts: i64) -> ThoughtNode {
        let id = format!("n{}", self.seq + 1);
        let node = ThoughtNode {
            detail: Some(truncate(text, 600)),
            status: NodeStatus::Ok,
            ..self.node(id, NodeKind::Notice, "other", &truncate(text, 80), ts)
        };
        self.push(node, true)
    }

    pub fn stop(&mut self, message: Option<&str>, ts: i64) -> ThoughtNode {
        let node = ThoughtNode {
            detail: message.map(|m| truncate(m, 2000)),
            status: NodeStatus::Ok,
            ..self.node(format!("s{}", self.turn), NodeKind::Stop, "think", "Listo", ts)
        };
        self.push(node, true)
    }

    fn node(&mut self, id: String, kind: NodeKind, family: &str, label: &str, ts: i64) -> ThoughtNode {
        self.seq += 1;
        ThoughtNode {
            id,
            turn: self.turn,
            seq: self.seq,
            kind,
            family: family.to_string(),
            tool: None,
            label: label.to_string(),
            detail: None,
            intent: None,
            target: None,
            status: NodeStatus::Ok,
            parent: self.tail.clone(),
            ts,
            duration_ms: None,
        }
    }

    fn push(&mut self, node: ThoughtNode, completes: bool) -> ThoughtNode {
        if completes {
            self.tail = Some(node.id.clone());
        }
        self.nodes.push(node.clone());

        if self.nodes.len() > MAX_NODES {
            // Descarta el turno más antiguo completo para no dejar aristas rotas.
            let oldest = self.nodes[0].turn;
            self.nodes.retain(|n| n.turn != oldest);
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn describes_common_tools() {
        let read = describe_tool("Read", &json!({ "file_path": "C:\\proj\\src\\main.rs" }));
        assert_eq!(read.family, "read");
        assert_eq!(read.label, "Leer main.rs");

        let bash = describe_tool("Bash", &json!({ "command": "npm test\n--watch" }));
        assert_eq!(bash.family, "exec");
        assert_eq!(bash.label, "npm test");
    }

    #[test]
    fn parallel_tools_share_a_parent() {
        let mut graph = GraphBuilder::default();
        let prompt = graph.begin_turn("arregla el bug", 1);
        let a = graph.tool_start(Some("a"), "Read", &json!({ "file_path": "/x/a.ts" }), 2);
        let b = graph.tool_start(Some("b"), "Read", &json!({ "file_path": "/x/b.ts" }), 2);
        assert_eq!(a.parent.as_deref(), Some(prompt.id.as_str()));
        assert_eq!(b.parent, a.parent);

        graph.tool_end(Some("a"), 5);
        let edit = graph.tool_start(Some("c"), "Edit", &json!({ "file_path": "/x/a.ts" }), 6);
        assert_eq!(edit.parent.as_deref(), Some("a"));
    }

    #[test]
    fn intent_can_arrive_before_the_node() {
        let mut graph = GraphBuilder::default();
        graph.begin_turn("hola", 1);
        assert!(graph.set_intent("t1", "Voy a leer el archivo").is_none());
        let node = graph.tool_start(Some("t1"), "Read", &json!({ "file_path": "/x/a.ts" }), 2);
        assert_eq!(node.intent.as_deref(), Some("Voy a leer el archivo"));
    }

    #[test]
    fn errors_are_reported_once() {
        let mut graph = GraphBuilder::default();
        graph.begin_turn("hola", 1);
        graph.tool_start(Some("t1"), "Bash", &json!({ "command": "false" }), 2);
        graph.tool_end(Some("t1"), 3);
        assert!(graph.mark_error("t1").is_some());
        assert!(graph.mark_error("t1").is_none());
    }
}
