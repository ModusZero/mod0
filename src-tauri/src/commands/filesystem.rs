use futures::future::{BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Option<Vec<FileNode>>,
}

// Este es el comando que Tauri ve
#[tauri::command]
pub async fn read_folder_recursive(path: PathBuf) -> Result<Vec<FileNode>, String> {
    read_folder(path).await
}

// Esta es la lógica recursiva real "Boxeada" para evitar tamaño infinito
fn read_folder(path: PathBuf) -> BoxFuture<'static, Result<Vec<FileNode>, String>> {
    async move {
        let mut nodes = Vec::new();

        // Leemos el directorio de forma síncrona (está bien para FS local)
        let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;

        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let metadata = entry.metadata().map_err(|e| e.to_string())?;
            let path = entry.path();
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let children = if metadata.is_dir() {
                Some(read_folder(path.clone()).await?)
            } else {
                None
            };

            nodes.push(FileNode {
                name,
                path,
                is_dir: metadata.is_dir(),
                children,
            });
        }

        // Ordenar: Carpetas primero, luego archivos
        nodes.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name)));

        Ok(nodes)
    }
    .boxed()
}

#[tauri::command]
pub async fn read_file_content(path: PathBuf) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_file(path: PathBuf, content: String) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}
