use std::path::PathBuf;
use std::sync::Arc;
use crate::infrastructure::events;
use crate::infrastructure::extension_host::ExtensionHost;
use crate::infrastructure::filesystem::io::FileIO;
use serde_json::json;

pub struct FileSystemDispatcher {
    extension_host: Arc<ExtensionHost>,
    // Otros como indexer, watcher...
}

impl FileSystemDispatcher {
    pub async fn save_file(&self, path: PathBuf, mut content: String) -> Result<(), String> {
        // 1. EVENTO "WILL_SAVE": Las extensiones pueden modificar el contenido (ej. Prettier)
        let payload = json!({ "path": path, "content": content });
        if let Ok(modified_content) = self.extension_host
            .dispatch_with_result(events::fs::ON_WILL_SAVE, payload).await {
                if let Some(new_text) = modified_content.as_str() {
                    content = new_text.to_string();
                }
        }

        // 2. FALLBACK LOCAL: Guardar en el disco
        FileIO::write(&path, &content)?;

        // 3. EVENTO "DID_SAVE": Notificar (ej. para disparar un Linter o Build automático)
        let _ = self.extension_host.dispatch(events::fs::ON_DID_SAVE, json!({ "path": path })).await;

        Ok(())
    }

    pub fn read_file(&self, path: PathBuf) -> Result<String, String> {
        FileIO::read(&path)
    }
}