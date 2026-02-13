use tokio::sync::mpsc;
use std::path::PathBuf;
use std::fs;
use ignore::WalkBuilder;
use crate::database::DbManager;
use crate::services::filesystem::scanner::FileCategorizer;
use crate::services::filesystem::nodes::FileCategory;

/// Tareas de procesamiento asíncrono para el índice de archivos.
pub enum IndexingTask {
    /// Indexa o actualiza un archivo individual.
    IndexFile(PathBuf),
    /// Elimina un archivo o una rama de directorios del índice.
    RemoveFile(PathBuf),
    /// Realiza un escaneo completo respetando .scanignore.
    FullScan(PathBuf),
}

pub struct IndexingWorker;

impl IndexingWorker {
    /// Inicia el worker asíncrono que escucha tareas de indexación.
    pub fn spawn(db: DbManager) -> mpsc::Sender<IndexingTask> {
        let (tx, mut rx) = mpsc::channel::<IndexingTask>(100);

        tokio::spawn(async move {
            let repo = db.repository();
            while let Some(task) = rx.recv().await {
                match task {
                    IndexingTask::IndexFile(path) => {
                        if let Ok(content) = fs::read_to_string(&path) {
                            let _ = repo.index_file(&path.to_string_lossy(), &content).await;
                        }
                    }
                    IndexingTask::RemoveFile(path) => {
                        // Importante: El repo debe usar LIKE 'path%' para borrar carpetas.
                        let _ = repo.delete_from_index(&path.to_string_lossy()).await;
                    }
                    IndexingTask::FullScan(root) => {
                        let categorizer = FileCategorizer::new(&root);
                        let walker = WalkBuilder::new(root).hidden(false).build();

                        for result in walker.flatten() {
                            let path = result.path();
                            let is_dir = path.is_dir();
                            let category = categorizer.categorize(path, is_dir);

                            // Solo indexamos contenido si es código fuente u oculto relevante (Source/Hidden).
                            if !is_dir && (category == FileCategory::Source || category == FileCategory::Hidden) {
                                if let Ok(content) = fs::read_to_string(path) {
                                    let _ = repo.index_file(&path.to_string_lossy(), &content).await;
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