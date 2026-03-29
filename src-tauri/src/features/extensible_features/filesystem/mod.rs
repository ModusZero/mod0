pub mod default;
pub mod internal_logic;

use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;
use default::io_filesystem::DefaultFileIO;

use internal_logic::{
    worker::IndexingTask,
    search::FileSearch,
    scanner::{FileScanner, FileNode}
};
use crate::persistence::database::{DbManager, models::skill::Skill};

pub mod filesystem_events {
    pub const ON_WILL_SAVE: &str = "fs.onWillSave";
    pub const ON_DID_SAVE: &str = "fs.onDidSave";
    pub const ON_DID_CREATE: &str = "fs.onDidCreate";
    pub const ON_DID_DELETE: &str = "fs.onDidDelete";
}

pub struct FileSystem {
    task_tx: mpsc::Sender<IndexingTask>,
    db: DbManager,
    scanner: Arc<FileScanner>,
}

// TODO implementarlo de forma que para cada accion busque si hay extensiones activas que contribuyan a ellas.
// De haber varias, debe buscar la de mayor prioridad, y delegarle la tarea, pero sincronizando de
// igual los `shared behaviors`

// Algo como:

// use std::path::PathBuf;
// use std::sync::Arc;
// use crate::infrastructure::events;
// use crate::infrastructure::extensions::ExtensionHost;
// use crate::infrastructure::filesystem::io::FileIO;
// use serde_json::json;

// pub struct FileSystemDispatcher {
//     extension_host: Arc<ExtensionHost>,
//     // Otros como indexer, watcher...
// }

// impl FileSystemDispatcher {
//     pub async fn save_file(&self, path: PathBuf, mut content: String) -> Result<(), String> {
//         // 1. EVENTO "WILL_SAVE": Las extensiones pueden modificar el contenido (ej. Prettier)
//         let payload = json!({ "path": path, "content": content });
//         if let Ok(modified_content) = self.extension_host
//             .dispatch_with_result(events::fs::ON_WILL_SAVE, payload).await {
//                 if let Some(new_text) = modified_content.as_str() {
//                     content = new_text.to_string();
//                 }
//         }

//         // 2. FALLBACK LOCAL: Guardar en el disco
//         FileIO::write(&path, &content)?;

//         // 3. EVENTO "DID_SAVE": Notificar (ej. para disparar un Linter o Build automático)
//         let _ = self.extension_host.dispatch(events::fs::ON_DID_SAVE, json!({ "path": path })).await;

//         Ok(())
//     }

//     pub fn read_file(&self, path: PathBuf) -> Result<String, String> {
//         FileIO::read(&path)
//     }
// }

impl FileSystem {
    pub fn new(task_tx: mpsc::Sender<IndexingTask>, db: DbManager, scanner: Arc<FileScanner>) -> Self {
        Self { task_tx, db, scanner }
    }

    pub fn save_file(&self, path: PathBuf, project_path: PathBuf, content: String) -> Result<(), String> {
        DefaultFileIO::write(&path, &content)?;
        let _ = self.task_tx.try_send(IndexingTask::IndexFile { path, project_path });
        Ok(())
    }

    pub fn create_file(&self, path: PathBuf, project_path: PathBuf) -> Result<(), String> {
        DefaultFileIO::write(&path, "")?;
        let _ = self.task_tx.try_send(IndexingTask::IndexFile { path, project_path });
        Ok(())
    }

    pub fn create_directory(&self, path: PathBuf) -> Result<(), String> {
        DefaultFileIO::create_dir(&path)
    }

    pub fn delete_item(&self, path: PathBuf, project_path: PathBuf) -> Result<(), String> {
        DefaultFileIO::delete(&path)?;
        let _ = self.task_tx.try_send(IndexingTask::RemoveFile { path, project_path });
        Ok(())
    }

    pub fn rename_item(&self, old: PathBuf, new: PathBuf, project_path: PathBuf) -> Result<(), String> {
        DefaultFileIO::rename(&old, &new)?;
        let _ = self.task_tx.try_send(IndexingTask::RemoveFile { path: old, project_path: project_path.clone() });
        
        if new.is_dir() {
            let _ = self.task_tx.try_send(IndexingTask::FullScan(new));
        } else {
            let _ = self.task_tx.try_send(IndexingTask::IndexFile { path: new, project_path });
        }
        Ok(())
    }

    pub fn read_file(&self, path: PathBuf) -> Result<String, String> {
        DefaultFileIO::read(&path)
    }

    pub fn exists(&self, path: PathBuf) -> bool {
        DefaultFileIO::exists(&path)
    }

    pub async fn get_tree(&self, root: PathBuf) -> Result<Vec<FileNode>, String> {
        self.scanner.clone().build_tree(root).await
    }

    pub async fn search_code(&self, query: &str, project_path: &str) -> Result<Vec<PathBuf>, String> {
        FileSearch::find_code(&self.db, query, project_path).await
    }

    pub async fn search_filename(&self, query: &str, project_path: &str) -> Result<Vec<PathBuf>, String> {
        FileSearch::find_filename(&self.db, query, project_path).await
    }

    pub async fn search_skills(&self, tags: Vec<String>) -> Result<Vec<Skill>, String> {
        FileSearch::find_skills_by_tags(&self.db, tags).await
    }

    pub fn sync_all(&self, project_path: PathBuf) -> Result<(), String> {
        self.task_tx.try_send(IndexingTask::FullScan(project_path))
            .map_err(|e| e.to_string())
    }
}