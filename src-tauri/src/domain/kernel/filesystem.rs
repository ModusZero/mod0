use crate::infrastructure::filesystem::*;

use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;
use io::FileIO;
use worker::IndexingTask;
use search::FileSearch;
use scanner::{FileScanner, FileNode};
use crate::database::{DbManager, models::intelligence::skill::Skill};

pub struct FileSystem {
    task_tx: mpsc::Sender<IndexingTask>,
    db: DbManager,
    scanner: Arc<FileScanner>,
}

impl FileSystem {
    pub fn new(task_tx: mpsc::Sender<IndexingTask>, db: DbManager, scanner: Arc<FileScanner>) -> Self {
        Self { task_tx, db, scanner }
    }

    pub fn save_file(&self, path: PathBuf, project_path: PathBuf, content: String) -> Result<(), String> {
        FileIO::write(&path, &content)?;
        let _ = self.task_tx.try_send(IndexingTask::IndexFile { path, project_path });
        Ok(())
    }

    pub fn create_file(&self, path: PathBuf, project_path: PathBuf) -> Result<(), String> {
        FileIO::write(&path, "")?;
        let _ = self.task_tx.try_send(IndexingTask::IndexFile { path, project_path });
        Ok(())
    }

    pub fn create_directory(&self, path: PathBuf) -> Result<(), String> {
        FileIO::create_dir(&path)
    }

    pub fn delete_item(&self, path: PathBuf, project_path: PathBuf) -> Result<(), String> {
        FileIO::delete(&path)?;
        let _ = self.task_tx.try_send(IndexingTask::RemoveFile { path, project_path });
        Ok(())
    }

    pub fn rename_item(&self, old: PathBuf, new: PathBuf, project_path: PathBuf) -> Result<(), String> {
        FileIO::rename(&old, &new)?;
        let _ = self.task_tx.try_send(IndexingTask::RemoveFile { path: old, project_path: project_path.clone() });
        
        if new.is_dir() {
            let _ = self.task_tx.try_send(IndexingTask::FullScan(new));
        } else {
            let _ = self.task_tx.try_send(IndexingTask::IndexFile { path: new, project_path });
        }
        Ok(())
    }

    pub fn read_file(&self, path: PathBuf) -> Result<String, String> {
        FileIO::read(&path)
    }

    pub fn exists(&self, path: PathBuf) -> bool {
        FileIO::exists(&path)
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