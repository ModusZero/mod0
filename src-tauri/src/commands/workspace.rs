use std::path::PathBuf;

use tauri::State;

use crate::core::error::AppResult;
use crate::core::state::AppState;
use crate::features::workspace::deps::{self, DepGraph};
use crate::features::workspace::io;
use crate::features::workspace::tree::{self, DirEntryInfo};
use crate::infrastructure::process::blocking;

/// Límite por defecto de archivos en el índice de búsqueda rápida.
const DEFAULT_FILE_LIMIT: usize = 30_000;

#[tauri::command]
pub async fn fs_list_dir(_state: State<'_, AppState>, path: String) -> AppResult<Vec<DirEntryInfo>> {
    blocking(move || tree::list_dir(&PathBuf::from(path))).await
}

#[tauri::command]
pub async fn fs_list_files(
    _state: State<'_, AppState>,
    root: String,
    limit: Option<usize>,
) -> AppResult<Vec<String>> {
    blocking(move || tree::list_files(&PathBuf::from(root), limit.unwrap_or(DEFAULT_FILE_LIMIT))).await
}

#[tauri::command]
pub async fn fs_read(_state: State<'_, AppState>, path: String) -> AppResult<String> {
    blocking(move || io::read_text(&PathBuf::from(path))).await
}

#[tauri::command]
pub async fn fs_write(_state: State<'_, AppState>, path: String, content: String) -> AppResult<()> {
    blocking(move || io::write_text(&PathBuf::from(path), &content)).await
}

#[tauri::command]
pub async fn fs_create_file(_state: State<'_, AppState>, path: String) -> AppResult<()> {
    blocking(move || io::create_file(&PathBuf::from(path))).await
}

#[tauri::command]
pub async fn fs_create_dir(_state: State<'_, AppState>, path: String) -> AppResult<()> {
    blocking(move || io::create_dir(&PathBuf::from(path))).await
}

#[tauri::command]
pub async fn fs_rename(_state: State<'_, AppState>, from: String, to: String) -> AppResult<()> {
    blocking(move || io::rename(&PathBuf::from(from), &PathBuf::from(to))).await
}

#[tauri::command]
pub async fn fs_delete(_state: State<'_, AppState>, path: String) -> AppResult<()> {
    blocking(move || io::delete(&PathBuf::from(path))).await
}

#[tauri::command]
pub async fn fs_graph(
    _state: State<'_, AppState>,
    root: String,
    expanded: Vec<String>,
    refresh: bool,
) -> AppResult<DepGraph> {
    blocking(move || deps::graph(&PathBuf::from(root), &expanded, refresh)).await
}

/// Archivos que dependen (directa o indirectamente) de los archivos indicados.
#[tauri::command]
pub async fn fs_impact(
    _state: State<'_, AppState>,
    root: String,
    paths: Vec<String>,
) -> AppResult<Vec<String>> {
    blocking(move || deps::impact(&PathBuf::from(root), &paths)).await
}
