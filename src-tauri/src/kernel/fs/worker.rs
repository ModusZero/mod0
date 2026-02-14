use tokio::sync::mpsc;
use std::path::{Path, PathBuf};
use std::fs;
use ignore::WalkBuilder;
use crate::database::DbManager;
use crate::kernel::fs::scanner::FileCategorizer;
use crate::kernel::fs::nodes::FileCategory;

/// Tareas procesables por el trabajador de segundo plano.
pub enum IndexingTask {
    /// Indexar o actualizar un archivo específico.
    IndexFile { path: PathBuf, project_path: PathBuf },
    /// Eliminar una entrada del índice.
    RemoveFile { path: PathBuf, project_path: PathBuf },
    /// Realizar un escaneo completo de un directorio.
    FullScan(PathBuf),
}

/// Gestor de tareas de persistencia e indexación de búsqueda.
pub struct IndexingWorker;

impl IndexingWorker {
    /// Inicia el bucle de eventos del worker en un hilo asíncrono.
    pub fn spawn(db: DbManager) -> mpsc::Sender<IndexingTask> {
        let (tx, mut rx) = mpsc::channel::<IndexingTask>(500);

        tokio::spawn(async move {
            while let Some(task) = rx.recv().await {
                let repo = db.repository();
                match task {
                    IndexingTask::IndexFile { path, project_path } => {
                        let _ = Self::process_file(&path, &project_path, &repo).await;
                    }
                    IndexingTask::RemoveFile { path, project_path } => {
                        let _ = repo.delete_from_index(&path.to_string_lossy(), &project_path.to_string_lossy()).await;
                    }
                    IndexingTask::FullScan(root) => {
                        let categorizer = FileCategorizer::new(&root);
                        let walker = WalkBuilder::new(&root).hidden(false).build();
                        for result in walker.flatten() {
                            let path = result.path();
                            if path.is_file() {
                                let cat = categorizer.categorize(path, false);
                                if matches!(cat, FileCategory::Source | FileCategory::Hidden) {
                                    let _ = Self::process_file(path, &root, &repo).await;
                                }
                            }
                        }
                    }
                }
            }
        });
        tx
    }

    async fn process_file(path: &Path, project_root: &Path, repo: &crate::database::Repository<'_>) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        repo.index_file(&path.to_string_lossy(), &project_root.to_string_lossy(), &content)
            .await
            .map_err(|e| e.to_string())
    }
}