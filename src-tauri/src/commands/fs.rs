use tauri::State;
use std::path::PathBuf;
use tokio::sync::mpsc;
use crate::database::DbManager;
use crate::kernel::fs::{nodes::FileNode, scanner, operations, worker::IndexingTask, search::SearchService};

/// Carga el árbol de archivos. 'path' es la carpeta a leer, 'root' es la raíz del workspace para cargar reglas.
#[tauri::command]
pub async fn get_file_tree(path: PathBuf, root: PathBuf) -> Result<Vec<FileNode>, String> {
    scanner::build_tree_recursive(path, root).await
}

/// Lee un archivo del disco.
#[tauri::command]
pub async fn read_file(path: PathBuf) -> Result<String, String> {
    operations::read_to_string(path)
}

/// Guarda un archivo existente y actualiza el índice.
#[tauri::command]
pub async fn write_file(path: PathBuf, content: String, tx: State<'_, mpsc::Sender<IndexingTask>>) -> Result<(), String> {
    operations::save_to_file(path, content, &tx)
}

/// Crea un nuevo archivo en blanco.
#[tauri::command]
pub async fn create_new_file(path: PathBuf, tx: State<'_, mpsc::Sender<IndexingTask>>) -> Result<(), String> {
    operations::create_file(path, &tx)
}

/// Crea una carpeta.
#[tauri::command]
pub async fn create_new_dir(path: PathBuf) -> Result<(), String> {
    operations::create_dir_all(path)
}

/// Elimina un archivo o carpeta.
#[tauri::command]
pub async fn delete_path(path: PathBuf, tx: State<'_, mpsc::Sender<IndexingTask>>) -> Result<(), String> {
    operations::delete_item(path, &tx)
}

/// Renombra o mueve un archivo/carpeta.
#[tauri::command]
pub async fn rename_path(old_path: PathBuf, new_path: PathBuf, tx: State<'_, mpsc::Sender<IndexingTask>>) -> Result<(), String> {
    operations::rename_item(old_path, new_path, &tx)
}

/// Búsqueda por contenido (FTS5).
#[tauri::command]
pub async fn global_search(db: State<'_, DbManager>, query: String) -> Result<Vec<PathBuf>, String> {
    SearchService::find_code(&db, &query).await
}

/// Búsqueda rápida por nombre de archivo.
#[tauri::command]
pub async fn quick_open(
    db: State<'_, DbManager>, 
    query: String
) -> Result<Vec<PathBuf>, String> {
    SearchService::find_filename(&db, &query).await
}