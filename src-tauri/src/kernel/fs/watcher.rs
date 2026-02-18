use notify::{Watcher, RecursiveMode, EventKind};
use std::path::PathBuf;
use tokio::sync::mpsc;
use crate::kernel::fs::worker::IndexingTask;
use tauri::{AppHandle, Emitter};

/// Observador de eventos del sistema de archivos con integración a Tauri.
pub struct FSWatcher {
    watcher: notify::RecommendedWatcher,
}

impl FSWatcher {
    /// Inicializa el observador y configura el callback de eventos.
    pub fn new(app_handle: AppHandle, index_tx: mpsc::Sender<IndexingTask>, project_path: PathBuf) -> Result<Self, notify::Error> {
        let watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            if let Ok(event) = res {
                let tx = index_tx.clone();
                let handle = app_handle.clone();
                let p_path = project_path.clone();

                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        for path in event.paths {
                            if path.is_file() {
                                let _ = tx.try_send(IndexingTask::IndexFile { path: path.clone(), project_path: p_path.clone() });
                                let _ = handle.emit("fs:changed", path);
                            }
                        }
                    }
                    EventKind::Remove(_) => {
                        for path in event.paths {
                            let _ = tx.try_send(IndexingTask::RemoveFile { path: path.clone(), project_path: p_path.clone() });
                            let _ = handle.emit("fs:removed", path);
                        }
                    }
                    _ => {}
                }
            }
        })?;
        Ok(Self { watcher })
    }

    /// Comienza a observar un directorio de forma recursiva.
    pub fn watch(&mut self, path: &PathBuf) -> Result<(), notify::Error> {
        self.watcher.watch(path, RecursiveMode::Recursive)
    }
}