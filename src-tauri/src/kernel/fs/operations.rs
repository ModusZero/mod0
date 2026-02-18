use std::fs;
use std::path::PathBuf;
use tokio::sync::mpsc;
use crate::kernel::fs::worker::IndexingTask;

/// Recupera el contenido de un archivo como cadena de texto.
pub fn read_to_string(path: PathBuf) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

/// Persiste contenido en disco y notifica al trabajador de indexación.
pub fn save_to_file(path: PathBuf, project_path: PathBuf, content: String, tx: &mpsc::Sender<IndexingTask>) -> Result<(), String> {
    fs::write(&path, &content).map_err(|e| e.to_string())?;
    let _ = tx.try_send(IndexingTask::IndexFile { path, project_path });
    Ok(())
}

/// Crea un archivo vacío en el sistema y lo registra en el índice.
pub fn create_file(path: PathBuf, project_path: PathBuf, tx: &mpsc::Sender<IndexingTask>) -> Result<(), String> {
    fs::write(&path, "").map_err(|e| e.to_string())?;
    let _ = tx.try_send(IndexingTask::IndexFile { path, project_path });
    Ok(())
}

/// Crea una estructura de directorios recursiva.
pub fn create_dir_all(path: PathBuf) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| e.to_string())
}

/// Elimina un recurso del disco y solicita su remoción del índice.
pub fn delete_item(path: PathBuf, project_path: PathBuf, tx: &mpsc::Sender<IndexingTask>) -> Result<(), String> {
    if path.is_dir() {
        fs::remove_dir_all(&path).map_err(|e| e.to_string())?;
    } else {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    let _ = tx.try_send(IndexingTask::RemoveFile { path, project_path });
    Ok(())
}

/// Renombra un recurso y actualiza su estado en el sistema de indexación.
pub fn rename_item(old_path: PathBuf, new_path: PathBuf, project_path: PathBuf, tx: &mpsc::Sender<IndexingTask>) -> Result<(), String> {
    fs::rename(&old_path, &new_path).map_err(|e| e.to_string())?;
    let _ = tx.try_send(IndexingTask::RemoveFile { path: old_path, project_path: project_path.clone() });
    
    if new_path.is_dir() {
        let _ = tx.try_send(IndexingTask::FullScan(new_path));
    } else {
        let _ = tx.try_send(IndexingTask::IndexFile { path: new_path, project_path });
    }
    Ok(())
}