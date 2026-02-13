use tokio::sync::mpsc;
use std::path::PathBuf;
use std::fs;
use ignore::WalkBuilder;
use crate::database::DbManager;
use crate::database::models::files::FileIndex;
use crate::kernel::fs::scanner::FileCategorizer;
use crate::kernel::fs::nodes::FileCategory;

pub enum IndexingTask {
    IndexFile(PathBuf),
    RemoveFile(PathBuf),
    FullScan(PathBuf),
}

pub struct IndexingWorker;

impl IndexingWorker {
    pub fn spawn(db: DbManager) -> mpsc::Sender<IndexingTask> {
        let (tx, mut rx) = mpsc::channel::<IndexingTask>(100);

        tokio::spawn(async move {
            let repo = db.repository();
            while let Some(task) = rx.recv().await {
                match task {
                    IndexingTask::IndexFile(path) => {
                        if let Ok(content) = fs::read_to_string(&path) {
                            let file_idx = FileIndex {
                                path: path.to_string_lossy().into_owned(),
                                content,
                            };
                            let _ = repo.index_file(&file_idx).await;
                        }
                    }
                    IndexingTask::RemoveFile(path) => {
                        let _ = repo.delete_from_index(&path.to_string_lossy()).await;
                    }
                    IndexingTask::FullScan(root) => {
                        let categorizer = FileCategorizer::new(&root);
                        let walker = WalkBuilder::new(root).hidden(false).build();

                        for result in walker.flatten() {
                            let path = result.path();
                            let is_dir = path.is_dir();
                            let category = categorizer.categorize(path, is_dir);

                            if !is_dir && (category == FileCategory::Source || category == FileCategory::Hidden) {
                                if let Ok(content) = fs::read_to_string(path) {
                                    let file_idx = FileIndex {
                                        path: path.to_string_lossy().into_owned(),
                                        content,
                                    };
                                    let _ = repo.index_file(&file_idx).await;
                                }
                            }
                        }
                    }
                }
            }
        });
        tx
    }
}