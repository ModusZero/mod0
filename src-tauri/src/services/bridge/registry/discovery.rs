use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashSet;
use crate::kernel::fs::scanner::FileCategorizer;
use crate::kernel::fs::nodes::FileCategory;
use crate::services::bridge::registry::catalog::ToolCatalog;
use crate::services::bridge::registry::defs::RuntimeDefinition;

pub struct DiscoveryEngine {
    project_root: PathBuf,
    mod0_dir: PathBuf,
    catalog: ToolCatalog,
}

impl DiscoveryEngine {
    pub fn new(root: &Path) -> Self {
        let mod0_dir = root.join(".mod0");
        if !mod0_dir.exists() { let _ = fs::create_dir_all(&mod0_dir); }

        Self {
            project_root: root.to_path_buf(),
            mod0_dir,
            catalog: ToolCatalog::load_internal(),
        }
    }

    pub fn run_smart_inference(&self) -> Vec<RuntimeDefinition> {
        let mut detected_ids = HashSet::new();
        let categorizer = FileCategorizer::new(&self.project_root);
        
        // Iniciamos escaneo recursivo limitado
        self.scan_recursive(&self.project_root, &categorizer, &mut detected_ids, 0);

        let mut suggestions: Vec<RuntimeDefinition> = detected_ids.into_iter()
            .filter_map(|id| self.catalog.definitions.get(&id).cloned())
            .collect();

        // Inyectar capacidades base obligatorias (como MCP Filesystem)
        if let Some(mcp_core) = self.catalog.definitions.get("mcp-fs") {
            suggestions.push(mcp_core.clone());
        }

        suggestions
    }

    fn scan_recursive(&self, path: &Path, cat: &FileCategorizer, found: &mut HashSet<String>, depth: u8) {
        if depth > 3 { return; }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                let is_dir = p.is_dir();
                let category = cat.categorize(&p, is_dir);

                // Si el categorizador dice que es ignorado/sistema, no entramos
                if matches!(category, FileCategory::Ignored | FileCategory::System) {
                    continue;
                }

                if is_dir {
                    self.scan_recursive(&p, cat, found, depth + 1);
                } else {
                    // Match por firma de archivo
                    if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                        if let Some(id) = self.catalog.signatures.get(name) {
                            found.insert(id.clone());
                        }
                    }
                    // Match por extensión
                    if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                        if let Some(id) = self.catalog.extensions.get(ext) {
                            found.insert(id.clone());
                        }
                    }
                }
            }
        }
    }

    pub fn save_to_capabilities(&self, defs: &[RuntimeDefinition]) -> Result<(), String> {
        let path = self.mod0_dir.join("capabilities.json");
        let json = serde_json::to_string_pretty(&defs).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }
}