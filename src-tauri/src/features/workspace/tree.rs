use std::path::Path;

use ignore::WalkBuilder;
use serde::Serialize;

use crate::core::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirEntryInfo {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

/// Un walker que respeta `.gitignore` aunque la carpeta no sea (todavía) un repositorio.
pub fn walker(root: &Path) -> WalkBuilder {
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(false)
        .require_git(false)
        .git_global(false)
        .filter_entry(|entry| entry.file_name() != ".git");
    builder
}

/// Contenido directo de una carpeta (carga perezosa: el árbol pide solo lo que expande).
pub fn list_dir(path: &Path) -> AppResult<Vec<DirEntryInfo>> {
    if !path.is_dir() {
        return Err(AppError::msg(format!("no es una carpeta: {}", path.display())));
    }

    let mut entries: Vec<DirEntryInfo> = walker(path)
        .max_depth(Some(1))
        .build()
        .flatten()
        .filter(|entry| entry.depth() == 1)
        .map(|entry| DirEntryInfo {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: entry.path().to_string_lossy().into_owned(),
            is_dir: entry.file_type().is_some_and(|kind| kind.is_dir()),
        })
        .collect();

    entries.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    Ok(entries)
}

/// Lista plana de archivos (rutas relativas con `/`) para la búsqueda rápida.
pub fn list_files(root: &Path, limit: usize) -> AppResult<Vec<String>> {
    let mut files = Vec::new();

    for entry in walker(root).build().flatten() {
        if entry.file_type().is_some_and(|kind| kind.is_file()) {
            if let Ok(relative) = entry.path().strip_prefix(root) {
                files.push(relative.to_string_lossy().replace('\\', "/"));
            }
            if files.len() >= limit {
                break;
            }
        }
    }
    Ok(files)
}
