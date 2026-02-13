use std::fs;
use std::path::{Path, PathBuf};
use futures::future::{BoxFuture, FutureExt};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use crate::kernel::fs::nodes::{FileNode, FileCategory};

/// Helper para clasificar archivos basándose en reglas de exclusión personalizadas y de Git.
pub struct FileCategorizer {
    gitignore: Option<Gitignore>,
    scanignore: Option<Gitignore>,
}

impl FileCategorizer {
    /// Inicializa un categorizador cargando los archivos de reglas desde la raíz del proyecto.
    pub fn new(project_root: &Path) -> Self {
        let mut git_builder = GitignoreBuilder::new(project_root);
        git_builder.add(project_root.join(".gitignore"));
        
        let mut scan_builder = GitignoreBuilder::new(project_root);
        scan_builder.add(project_root.join(".scanignore"));

        Self {
            gitignore: git_builder.build().ok(),
            scanignore: scan_builder.build().ok(),
        }
    }

    /// Evalúa la categoría de un path específico.
    pub fn categorize(&self, path: &Path, is_dir: bool) -> FileCategory {
        let name = path.file_name().unwrap_or_default().to_string_lossy();

        // 1. Prioridad: .scanignore define qué es "ruido de sistema" (System).
        if let Some(ref si) = self.scanignore {
            if si.matched(path, is_dir).is_ignore() {
                return FileCategory::System;
            }
        }

        // 2. .gitignore define qué está fuera del control de versiones (Ignored).
        if let Some(ref gi) = self.gitignore {
            if gi.matched(path, is_dir).is_ignore() {
                return FileCategory::Ignored;
            }
        }

        // 3. Archivos ocultos.
        if name.starts_with('.') {
            return FileCategory::Hidden;
        }

        FileCategory::Source
    }
}

/// Construye un árbol recursivo del sistema de archivos categorizando cada nodo.
pub async fn build_tree_recursive(path: PathBuf, root: PathBuf) -> Result<Vec<FileNode>, String> {
    let categorizer = std::sync::Arc::new(FileCategorizer::new(&root));
    build_tree_internal(path, categorizer).await
}

fn build_tree_internal(path: PathBuf, categorizer: std::sync::Arc<FileCategorizer>) -> BoxFuture<'static, Result<Vec<FileNode>, String>> {
    async move {
        let mut nodes = Vec::new();
        let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;

        for entry in entries.flatten() {
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            let entry_path = entry.path();
            let is_dir = metadata.is_dir();
            let name = entry_path.file_name().unwrap_or_default().to_string_lossy().to_string();

            let category = categorizer.categorize(&entry_path, is_dir);

            let children = if is_dir {
                Some(build_tree_internal(entry_path.clone(), categorizer.clone()).await?)
            } else {
                None
            };

            nodes.push(FileNode {
                name,
                path: entry_path,
                is_dir,
                category,
                children,
            });
        }
        
        nodes.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name)));
        Ok(nodes)
    }.boxed()
}