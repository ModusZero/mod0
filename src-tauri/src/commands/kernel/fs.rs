use tauri::State;
use std::path::PathBuf;
use tokio::sync::mpsc;
use crate::database::DbManager;
use crate::kernel::fs::{
    nodes::FileNode,
    scanner,
    operations,
    worker::IndexingTask,
    search::SearchService
};

/// Obtiene el árbol de archivos de forma recursiva aplicando las reglas de categorización.
#[tauri::command]
pub async fn get_file_tree(path: PathBuf, root: PathBuf) -> Result<Vec<FileNode>, String> {
    scanner::build_tree_recursive(path, root).await
}

/// Lee el contenido de un archivo desde el disco.
#[tauri::command]
pub async fn read_file(path: PathBuf) -> Result<String, String> {
    operations::read_to_string(path)
}

/// Guarda cambios en un archivo y solicita su re-indexación.
#[tauri::command]
pub async fn write_file(
    path: PathBuf,
    project_path: PathBuf,
    content: String, 
    is_ai_generated: bool,
    tx: State<'_, mpsc::Sender<IndexingTask>>
) -> Result<(), String> {
    if is_ai_generated {
        println!("IA editando archivo: {:?}", path);
    }
    
    // Se pasa project_path para que el indexador sepa a qué proyecto pertenece el archivo
    operations::save_to_file(path, project_path, content, &tx)
}

/// Crea un nuevo archivo vacío y lo registra en el sistema de búsqueda.
#[tauri::command]
pub async fn create_new_file(
    path: PathBuf, 
    project_path: PathBuf, 
    tx: State<'_, mpsc::Sender<IndexingTask>>
) -> Result<(), String> {
    operations::create_file(path, project_path, &tx)
}

/// Crea un directorio y todos sus padres necesarios.
#[tauri::command]
pub async fn create_new_dir(path: PathBuf) -> Result<(), String> {
    operations::create_dir_all(path)
}

/// Elimina un archivo o carpeta y limpia su rastro en el índice.
#[tauri::command]
pub async fn delete_path(
    path: PathBuf, 
    project_path: PathBuf, 
    tx: State<'_, mpsc::Sender<IndexingTask>>
) -> Result<(), String> {
    operations::delete_item(path, project_path, &tx)
}

/// Renombra un recurso y actualiza las rutas en la base de datos.
#[tauri::command]
pub async fn rename_path(
    old_path: PathBuf, 
    new_path: PathBuf, 
    project_path: PathBuf, 
    tx: State<'_, mpsc::Sender<IndexingTask>>
) -> Result<(), String> {
    operations::rename_item(old_path, new_path, project_path, &tx)
}

/// Realiza una búsqueda de texto completo (FTS5) en el contenido de los archivos del proyecto.
#[tauri::command]
pub async fn global_search(
    db: State<'_, DbManager>, 
    query: String, 
    project_path: String
) -> Result<Vec<PathBuf>, String> {
    SearchService::find_code(&db, &query, &project_path).await
}

/// Realiza una búsqueda rápida por nombre de archivo usando patrones LIKE.
#[tauri::command]
pub async fn quick_open(
    db: State<'_, DbManager>, 
    query: String, 
    project_path: String
) -> Result<Vec<PathBuf>, String> {
    SearchService::find_filename(&db, &query, &project_path).await
}