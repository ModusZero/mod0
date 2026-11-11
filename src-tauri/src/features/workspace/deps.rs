use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex};

use regex::Regex;
use serde::Serialize;

use super::tree::walker;
use crate::core::error::AppResult;

/// Tamaño máximo de un archivo fuente que se analiza.
const MAX_SOURCE_BYTES: u64 = 512 * 1024;
const MAX_FILES: usize = 20_000;
const SOURCE_EXTENSIONS: [&str; 13] = [
    "ts", "tsx", "js", "jsx", "mjs", "cjs", "svelte", "vue", "astro", "py", "rs", "css", "scss",
];
const JS_EXTENSIONS: [&str; 9] = ["ts", "tsx", "js", "jsx", "mjs", "cjs", "svelte", "vue", "astro"];

static JS_FROM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:import|export)\s+(?:[^'";]*?\s+from\s+)?['"]([^'"\n]+)['"]"#).expect("regex")
});
static JS_CALL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?:import|require)\(\s*['"]([^'"\n]+)['"]\s*\)"#).expect("regex"));
static PY_FROM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*from\s+([.\w]+)\s+import").expect("regex"));
static PY_IMPORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*import\s+([\w.]+)").expect("regex"));
static RS_MOD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;").expect("regex"));
static RS_USE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*(?:pub\s+)?use\s+crate::(\w+)").expect("regex"));

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub files: u32,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub weight: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepGraph {
    pub depth: u32,
    pub max_depth: u32,
    pub file_count: u32,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Grafo a nivel de archivo (se cachea; la agregación por carpeta es barata).
struct FileGraph {
    files: Vec<(String, u64)>,
    edges: Vec<(usize, usize)>,
}

static CACHE: LazyLock<Mutex<HashMap<PathBuf, Arc<FileGraph>>>> = LazyLock::new(Default::default);

/// Grafo de dependencias agregado por carpeta hasta `depth` niveles.
pub fn graph(root: &Path, depth: u32, refresh: bool) -> AppResult<DepGraph> {
    let cached = if refresh {
        None
    } else {
        CACHE.lock().expect("deps cache").get(root).cloned()
    };

    let file_graph = match cached {
        Some(graph) => graph,
        None => {
            let built = Arc::new(scan(root));
            CACHE
                .lock()
                .expect("deps cache")
                .insert(root.to_path_buf(), built.clone());
            built
        }
    };
    Ok(aggregate(&file_graph, depth.max(1)))
}

fn scan(root: &Path) -> FileGraph {
    let mut files: Vec<(String, u64)> = Vec::new();

    for entry in walker(root).build().flatten() {
        if !entry.file_type().is_some_and(|kind| kind.is_file()) {
            continue;
        }
        let ext = entry.path().extension().and_then(|e| e.to_str()).unwrap_or("");
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);

        if SOURCE_EXTENSIONS.contains(&ext) && size <= MAX_SOURCE_BYTES {
            if let Ok(relative) = entry.path().strip_prefix(root) {
                files.push((relative.to_string_lossy().replace('\\', "/"), size));
            }
        }
        if files.len() >= MAX_FILES {
            break;
        }
    }

    let index: HashMap<&str, usize> = files
        .iter()
        .enumerate()
        .map(|(i, (path, _))| (path.as_str(), i))
        .collect();

    let mut edges = Vec::new();
    for (from, (path, _)) in files.iter().enumerate() {
        let Ok(source) = fs::read_to_string(root.join(path)) else {
            continue;
        };
        for target in imports_of(path, &source, &index) {
            if target != from {
                edges.push((from, target));
            }
        }
    }
    edges.sort_unstable();
    edges.dedup();

    FileGraph { files, edges }
}

/// Archivos del proyecto a los que apunta `source` (resueltos contra `index`).
fn imports_of(path: &str, source: &str, index: &HashMap<&str, usize>) -> Vec<usize> {
    let ext = path.rsplit('.').next().unwrap_or("");
    let dir = path.rsplit_once('/').map(|(dir, _)| dir).unwrap_or("");
    let mut found = Vec::new();

    if JS_EXTENSIONS.contains(&ext) {
        for pattern in [&*JS_FROM, &*JS_CALL] {
            for capture in pattern.captures_iter(source) {
                if let Some(target) = resolve_js(&capture[1], dir, index) {
                    found.push(target);
                }
            }
        }
    } else if ext == "py" {
        for capture in PY_FROM
            .captures_iter(source)
            .chain(PY_IMPORT.captures_iter(source))
        {
            if let Some(target) = resolve_py(&capture[1], dir, index) {
                found.push(target);
            }
        }
    } else if ext == "rs" {
        for capture in RS_MOD.captures_iter(source) {
            if let Some(target) = resolve_rs_mod(&capture[1], path, dir, index) {
                found.push(target);
            }
        }
        for capture in RS_USE.captures_iter(source) {
            if let Some(target) = resolve_rs_use(&capture[1], dir, index) {
                found.push(target);
            }
        }
    }
    found
}

fn normalize(path: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                out.pop();
            }
            other => out.push(other),
        }
    }
    out.join("/")
}

fn join(dir: &str, rest: &str) -> String {
    if dir.is_empty() {
        normalize(rest)
    } else {
        normalize(&format!("{dir}/{rest}"))
    }
}

fn probe(base: &str, index: &HashMap<&str, usize>) -> Option<usize> {
    if let Some(found) = index.get(base) {
        return Some(*found);
    }
    // Los proyectos TS importan `./x.js` aunque el archivo sea `x.ts`.
    let stem = ["js", "jsx", "mjs", "cjs"]
        .iter()
        .find_map(|ext| base.strip_suffix(&format!(".{ext}")))
        .unwrap_or(base);

    JS_EXTENSIONS.iter().find_map(|ext| {
        index
            .get(format!("{stem}.{ext}").as_str())
            .or_else(|| index.get(format!("{base}.{ext}").as_str()))
            .or_else(|| index.get(format!("{base}/index.{ext}").as_str()))
            .copied()
    })
}

fn resolve_js(spec: &str, dir: &str, index: &HashMap<&str, usize>) -> Option<usize> {
    let base = if spec.starts_with('.') {
        join(dir, spec)
    } else if let Some(rest) = spec.strip_prefix("$lib/") {
        normalize(&format!("src/lib/{rest}"))
    } else if let Some(rest) = spec.strip_prefix("@/").or_else(|| spec.strip_prefix("~/")) {
        normalize(&format!("src/{rest}"))
    } else {
        return None;
    };
    probe(&base, index)
}

fn resolve_py(spec: &str, dir: &str, index: &HashMap<&str, usize>) -> Option<usize> {
    let dots = spec.chars().take_while(|c| *c == '.').count();
    let module = spec[dots..].replace('.', "/");

    let mut bases = Vec::new();
    if dots > 0 {
        let mut up = dir.to_string();
        for _ in 1..dots {
            up = up
                .rsplit_once('/')
                .map(|(parent, _)| parent.to_string())
                .unwrap_or_default();
        }
        bases.push(join(&up, &module));
    } else {
        bases.push(normalize(&module));
        bases.push(normalize(&format!("src/{module}")));
    }

    bases.iter().find_map(|base| {
        index
            .get(format!("{base}.py").as_str())
            .or_else(|| index.get(format!("{base}/__init__.py").as_str()))
            .copied()
    })
}

fn resolve_rs_mod(name: &str, path: &str, dir: &str, index: &HashMap<&str, usize>) -> Option<usize> {
    let file = path.rsplit('/').next().unwrap_or(path);
    let owner_dir = if matches!(file, "mod.rs" | "lib.rs" | "main.rs") {
        dir.to_string()
    } else {
        join(dir, file.trim_end_matches(".rs"))
    };

    index
        .get(join(&owner_dir, &format!("{name}.rs")).as_str())
        .or_else(|| index.get(join(&owner_dir, &format!("{name}/mod.rs")).as_str()))
        .copied()
}

fn resolve_rs_use(name: &str, dir: &str, index: &HashMap<&str, usize>) -> Option<usize> {
    // Sube hasta la carpeta `src` más cercana: es la raíz del crate.
    let mut cursor = dir;
    loop {
        if cursor == "src" || cursor.ends_with("/src") {
            return index
                .get(join(cursor, &format!("{name}.rs")).as_str())
                .or_else(|| index.get(join(cursor, &format!("{name}/mod.rs")).as_str()))
                .copied();
        }
        cursor = cursor.rsplit_once('/')?.0;
    }
}

fn group_of(path: &str, depth: u32) -> String {
    let dirs: Vec<&str> = path.split('/').collect();
    let dirs = &dirs[..dirs.len().saturating_sub(1)];
    if dirs.is_empty() {
        ".".to_string()
    } else {
        dirs[..dirs.len().min(depth as usize)].join("/")
    }
}

fn aggregate(graph: &FileGraph, depth: u32) -> DepGraph {
    let mut nodes: BTreeMap<String, GraphNode> = BTreeMap::new();
    let mut max_depth = 0;

    let groups: Vec<String> = graph
        .files
        .iter()
        .map(|(path, bytes)| {
            max_depth = max_depth.max(path.matches('/').count() as u32);
            let id = group_of(path, depth);
            let label = id.rsplit('/').next().unwrap_or(&id).to_string();
            let node = nodes.entry(id.clone()).or_insert(GraphNode {
                id: id.clone(),
                label,
                files: 0,
                bytes: 0,
            });
            node.files += 1;
            node.bytes += bytes;
            id
        })
        .collect();

    let mut weights: BTreeMap<(String, String), u32> = BTreeMap::new();
    for (from, to) in &graph.edges {
        let (a, b) = (&groups[*from], &groups[*to]);
        if a != b {
            *weights.entry((a.clone(), b.clone())).or_insert(0) += 1;
        }
    }

    DepGraph {
        depth,
        max_depth,
        file_count: graph.files.len() as u32,
        nodes: nodes.into_values().collect(),
        edges: weights
            .into_iter()
            .map(|((from, to), weight)| GraphEdge { from, to, weight })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index(paths: &[&'static str]) -> HashMap<&'static str, usize> {
        paths.iter().enumerate().map(|(i, p)| (*p, i)).collect()
    }

    #[test]
    fn resolves_relative_alias_and_ts_style_js_imports() {
        let idx = index(&[
            "src/lib/a.ts",
            "src/lib/b.svelte",
            "src/lib/ui/index.ts",
            "src/main.ts",
        ]);
        let source = "import a from './a.js';\nimport B from '$lib/b.svelte';\nexport * from './ui';\nimport x from 'react';";
        let mut found = imports_of("src/lib/c.ts", source, &idx);
        found.sort_unstable();
        assert_eq!(found, vec![0, 1, 2]);
    }

    #[test]
    fn resolves_rust_modules() {
        let idx = index(&[
            "src/lib.rs",
            "src/core/mod.rs",
            "src/core/error.rs",
            "src/config.rs",
        ]);
        assert_eq!(
            imports_of("src/lib.rs", "mod core;\nmod config;", &idx),
            vec![1, 3]
        );
        assert_eq!(
            imports_of("src/core/mod.rs", "pub mod error;\nuse crate::config::X;", &idx),
            vec![2, 3]
        );
    }

    #[test]
    fn aggregates_by_folder_depth() {
        let graph = FileGraph {
            files: vec![
                ("src/a/x.ts".into(), 10),
                ("src/a/y.ts".into(), 5),
                ("src/b/z.ts".into(), 1),
            ],
            edges: vec![(0, 2), (1, 2), (0, 1)],
        };
        let aggregated = aggregate(&graph, 2);
        assert_eq!(aggregated.nodes.len(), 2);
        assert_eq!(aggregated.edges.len(), 1);
        assert_eq!(aggregated.edges[0].weight, 2);
        assert_eq!(aggregate(&graph, 1).nodes.len(), 1);
    }
}
