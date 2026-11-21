use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
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
const MAX_DIRS: usize = 5_000;
const MAX_IMPACT: usize = 10_000;
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
    /// `dir` (carpeta agregada, expandible) o `file` (archivo individual).
    pub kind: &'static str,
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
    /// Profundidad máxima de carpetas del proyecto.
    pub max_depth: u32,
    pub file_count: u32,
    /// Todas las carpetas con código analizado (para expandir hasta un nivel dado).
    pub dirs: Vec<String>,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Grafo a nivel de archivo (se cachea; la agregación por carpeta es barata).
struct FileGraph {
    files: Vec<(String, u64)>,
    edges: Vec<(usize, usize)>,
}

static CACHE: LazyLock<Mutex<HashMap<PathBuf, Arc<FileGraph>>>> = LazyLock::new(Default::default);

/// Grafo a nivel de archivo del proyecto (cacheado por raíz; `refresh` fuerza un nuevo análisis).
fn file_graph(root: &Path, refresh: bool) -> Arc<FileGraph> {
    let cached = if refresh {
        None
    } else {
        CACHE.lock().expect("deps cache").get(root).cloned()
    };

    cached.unwrap_or_else(|| {
        let built = Arc::new(scan(root));
        CACHE
            .lock()
            .expect("deps cache")
            .insert(root.to_path_buf(), built.clone());
        built
    })
}

/// Grafo de dependencias agregado. Las carpetas de `expanded` se abren en sus hijos (carpetas o archivos);
/// lo demás queda colapsado en un solo nodo. Con `expanded` vacío se ve el primer nivel.
pub fn graph(root: &Path, expanded: &[String], refresh: bool) -> AppResult<DepGraph> {
    let open: HashSet<String> = expanded.iter().cloned().collect();
    Ok(aggregate(&file_graph(root, refresh), &open))
}

/// Archivos del proyecto que dependen (directa o indirectamente) de los archivos dados.
pub fn impact(root: &Path, paths: &[String]) -> AppResult<Vec<String>> {
    Ok(impact_of(&file_graph(root, false), paths))
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

/// Nodo al que pertenece un archivo: baja por las carpetas expandidas y se detiene en la primera colapsada.
fn group_of(path: &str, expanded: &HashSet<String>) -> String {
    let parts: Vec<&str> = path.split('/').collect();
    let mut prefix = String::new();

    for (index, part) in parts.iter().enumerate() {
        if !prefix.is_empty() {
            prefix.push('/');
        }
        prefix.push_str(part);

        if index == parts.len() - 1 || !expanded.contains(&prefix) {
            return prefix;
        }
    }
    prefix
}

fn aggregate(graph: &FileGraph, expanded: &HashSet<String>) -> DepGraph {
    let mut nodes: BTreeMap<String, GraphNode> = BTreeMap::new();
    let mut dirs: BTreeSet<String> = BTreeSet::new();
    let mut max_depth = 0;

    let groups: Vec<String> = graph
        .files
        .iter()
        .map(|(path, bytes)| {
            let depth = path.matches('/').count() as u32;
            max_depth = max_depth.max(depth);

            let parts: Vec<&str> = path.split('/').collect();
            for end in 1..parts.len() {
                dirs.insert(parts[..end].join("/"));
            }

            let id = group_of(path, expanded);
            let kind = if id == *path { "file" } else { "dir" };
            let label = id.rsplit('/').next().unwrap_or(&id).to_string();
            let node = nodes.entry(id.clone()).or_insert(GraphNode {
                id: id.clone(),
                label,
                files: 0,
                bytes: 0,
                kind,
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
        max_depth,
        file_count: graph.files.len() as u32,
        dirs: dirs.into_iter().take(MAX_DIRS).collect(),
        nodes: nodes.into_values().collect(),
        edges: weights
            .into_iter()
            .map(|((from, to), weight)| GraphEdge { from, to, weight })
            .collect(),
    }
}

/// Recorrido inverso: quién importa (directa o transitivamente) a los archivos semilla.
fn impact_of(graph: &FileGraph, seeds: &[String]) -> Vec<String> {
    let index: HashMap<&str, usize> = graph
        .files
        .iter()
        .enumerate()
        .map(|(i, (path, _))| (path.as_str(), i))
        .collect();

    let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); graph.files.len()];
    for (from, to) in &graph.edges {
        dependents[*to].push(*from);
    }

    let seed_ids: HashSet<usize> = seeds
        .iter()
        .filter_map(|path| index.get(path.as_str()).copied())
        .collect();
    let mut seen = seed_ids.clone();
    let mut queue: Vec<usize> = seed_ids.iter().copied().collect();

    while let Some(current) = queue.pop() {
        for dependent in &dependents[current] {
            if seen.insert(*dependent) {
                queue.push(*dependent);
            }
        }
    }

    let mut affected: Vec<String> = seen
        .difference(&seed_ids)
        .map(|i| graph.files[*i].0.clone())
        .take(MAX_IMPACT)
        .collect();
    affected.sort();
    affected
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

    fn sample() -> FileGraph {
        FileGraph {
            files: vec![
                ("src/a/x.ts".into(), 10),
                ("src/a/y.ts".into(), 5),
                ("src/b/z.ts".into(), 1),
                ("README.ts".into(), 1),
            ],
            // x -> z, y -> z, x -> y
            edges: vec![(0, 2), (1, 2), (0, 1)],
        }
    }

    #[test]
    fn collapsed_by_default_and_expandable_down_to_files() {
        let graph = sample();
        let closed = aggregate(&graph, &HashSet::new());
        let ids: Vec<&str> = closed.nodes.iter().map(|n| n.id.as_str()).collect();
        assert_eq!(ids, vec!["README.ts", "src"]);
        assert_eq!(closed.max_depth, 2);
        assert_eq!(closed.dirs, vec!["src", "src/a", "src/b"]);

        let open: HashSet<String> = ["src".to_string()].into();
        let folders = aggregate(&graph, &open);
        assert_eq!(folders.nodes.len(), 3);
        assert_eq!(folders.edges.len(), 1);
        assert_eq!(folders.edges[0].weight, 2);

        let files: HashSet<String> = ["src".to_string(), "src/a".to_string()].into();
        let deep = aggregate(&graph, &files);
        assert!(
            deep.nodes
                .iter()
                .any(|n| n.id == "src/a/x.ts" && n.kind == "file")
        );
        assert!(deep.nodes.iter().any(|n| n.id == "src/b" && n.kind == "dir"));
    }

    #[test]
    fn impact_follows_dependents_transitively() {
        let graph = sample();
        // z lo importan x e y; x importa a y, así que cambiar z afecta a x e y.
        assert_eq!(
            impact_of(&graph, &["src/b/z.ts".to_string()]),
            vec!["src/a/x.ts", "src/a/y.ts"]
        );
        assert_eq!(
            impact_of(&graph, &["src/a/x.ts".to_string()]),
            Vec::<String>::new()
        );
        assert_eq!(impact_of(&graph, &["nope.ts".to_string()]), Vec::<String>::new());
    }
}
