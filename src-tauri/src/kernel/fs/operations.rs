use std::fs;
use std::path::PathBuf;
use tokio::sync::mpsc;
use crate::kernel::fs::worker::IndexingTask;

/// Lee el contenido de un archivo a String.
pub fn read_to_string(path: PathBuf) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

/// Guarda contenido en un archivo e inicia la re-indexación.
pub fn save_to_file(path: PathBuf, content: String, tx: &mpsc::Sender<IndexingTask>) -> Result<(), String> {
    fs::write(&path, &content).map_err(|e| e.to_string())?;
    let _ = tx.try_send(IndexingTask::IndexFile(path));
    Ok(())
}

/// Crea un nuevo archivo vacío y lo registra en el índice.
pub fn create_file(path: PathBuf, tx: &mpsc::Sender<IndexingTask>) -> Result<(), String> {
    fs::write(&path, "").map_err(|e| e.to_string())?;
    let _ = tx.try_send(IndexingTask::IndexFile(path));
    Ok(())
}

/// Crea un directorio y todos sus padres si no existen.
pub fn create_dir_all(path: PathBuf) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| e.to_string())
}

/// Elimina un archivo o directorio recursivamente y limpia el índice.
pub fn delete_item(path: PathBuf, tx: &mpsc::Sender<IndexingTask>) -> Result<(), String> {
    if path.is_dir() {
        fs::remove_dir_all(&path).map_err(|e| e.to_string())?;
    } else {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    let _ = tx.try_send(IndexingTask::RemoveFile(path));
    Ok(())
}

/// Renombra o mueve un item, actualizando las referencias en la base de datos de búsqueda.
pub fn rename_item(old_path: PathBuf, new_path: PathBuf, tx: &mpsc::Sender<IndexingTask>) -> Result<(), String> {
    fs::rename(&old_path, &new_path).map_err(|e| e.to_string())?;
    
    // Notificamos la eliminación de la ruta antigua
    let _ = tx.try_send(IndexingTask::RemoveFile(old_path));
    
    // Indexamos la nueva ubicación
    if new_path.is_dir() {
        let _ = tx.try_send(IndexingTask::FullScan(new_path));
    } else {
        let _ = tx.try_send(IndexingTask::IndexFile(new_path));
    }
    Ok(())
}