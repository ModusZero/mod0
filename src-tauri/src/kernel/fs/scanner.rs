use std::fs;
use std::path::{Path, PathBuf};
use futures::future::{BoxFuture, FutureExt};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use crate::kernel::fs::nodes::{FileNode, FileCategory};
use std::sync::Arc;

/// Motor de clasificación de archivos basado en reglas de ignorado.
pub struct FileCategorizer {
    gitignore: Option<Gitignore>,
    scanignore: Option<Gitignore>,
}

impl FileCategorizer {
    /// Crea una nueva instancia cargando archivos .gitignore y .scanignore.
    pub fn new(project_root: &Path) -> Self {
        let mut git_builder = GitignoreBuilder::new(project_root);
        git_builder.add(project_root.join(".gitignore"));
        let mut scan_builder = GitignoreBuilder::new(project_root);
        scan_builder.add(project_root.join(".mod0/.scanignore"));

        Self {
            gitignore: git_builder.build().ok(),
            scanignore: scan_builder.build().ok(),
        }
    }

    /// Determina la categoría de un archivo según su nombre y reglas de exclusión.
    pub fn categorize(&self, path: &Path, is_dir: bool) -> FileCategory {
        if let Some(ref si) = self.scanignore {
            if si.matched(path, is_dir).is_ignore() { return FileCategory::System; }
        }
        if let Some(ref gi) = self.gitignore {
            if gi.matched(path, is_dir).is_ignore() { return FileCategory::Ignored; }
        }
        if path.file_name().map_or(false, |n| n.to_string_lossy().starts_with('.')) {
            return FileCategory::Hidden;
        }
        FileCategory::Source
    }
}

/// Genera un árbol de archivos de forma asíncrona y recursiva.
pub async fn build_tree_recursive(path: PathBuf, root: PathBuf) -> Result<Vec<FileNode>, String> {
    let categorizer = Arc::new(FileCategorizer::new(&root));
    build_tree_internal(path, categorizer).await
}

fn build_tree_internal(path: PathBuf, categorizer: Arc<FileCategorizer>) -> BoxFuture<'static, Result<Vec<FileNode>, String>> {
    async move {
        let mut nodes = Vec::new();
        let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;

        for entry in entries.flatten() {
            let entry_path = entry.path();
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            let is_dir = metadata.is_dir();
            let category = categorizer.categorize(&entry_path, is_dir);

            let children = if is_dir {
                Some(build_tree_internal(entry_path.clone(), categorizer.clone()).await?)
            } else {
                None
            };

            nodes.push(FileNode {
                name: entry_path.file_name().unwrap_or_default().to_string_lossy().to_string(),
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