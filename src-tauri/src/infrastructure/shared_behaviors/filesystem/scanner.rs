// scanner.rs
use std::fs;
use std::path::{Path, PathBuf};
use futures::future::{BoxFuture, FutureExt};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

/// Categoría de un nodo para definir su comportamiento en la interfaz y motor de búsqueda.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileCategory {
    /// Archivos de código fuente indexables.
    Source,
    /// Archivos o Carpetas con prefijo punto.
    Hidden,
    /// Archivos excluidos por reglas de Git.
    Ignored,
    /// Archivos de sistema o configuración interna de la aplicación.
    System,
}

/// Estructura jerárquica que representa una entrada en el sistema de archivos.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileNode {
    /// Nombre base del archivo o carpeta.
    pub name: String,
    /// Ruta absoluta en el sistema de archivos.
    pub path: PathBuf,
    /// Define si el nodo es una carpeta.
    pub is_dir: bool,
    /// Clasificación basada en reglas de filtrado.
    pub category: FileCategory,
    /// Lista de nodos hijos en caso de ser un directorio.
    pub children: Option<Vec<FileNode>>,
}

pub struct FileScanner {
    pub categorizer: Arc<FileCategorizer>,
}

impl FileScanner {
    pub fn new(project_root: &Path) -> Self {
        Self {
            categorizer: Arc::new(FileCategorizer::new(project_root)),
        }
    }

    pub async fn build_tree(self: Arc<Self>, path: PathBuf) -> Result<Vec<FileNode>, String> {
        Self::build_tree_recursive(self.clone(), path).await
    }

    /// Función asociada que usa Arc en lugar de &self para evitar problemas de lifetime
    fn build_tree_recursive(scanner: Arc<Self>, path: PathBuf) -> BoxFuture<'static, Result<Vec<FileNode>, String>> {
        async move {
            let mut nodes = Vec::new();
            // Leemos el directorio de forma síncrona (fs::read_dir es bloqueante, 
            // pero aceptable en workers si no saturas el runtime)
            let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;

            for entry in entries.flatten() {
                let entry_path = entry.path();
                let metadata = entry.metadata().map_err(|e| e.to_string())?;
                let is_dir = metadata.is_dir();
                
                // Usamos el categorizer a través del Arc
                let category = scanner.categorizer.categorize(&entry_path, is_dir);

                let children = if is_dir {
                    // Clonamos el Arc del scanner para la siguiente iteración recursiva
                    Some(Self::build_tree_recursive(scanner.clone(), entry_path.clone()).await?)
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
}

pub struct FileCategorizer {
    gitignore: Option<Gitignore>,
    scanignore: Option<Gitignore>,
}

impl FileCategorizer {
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