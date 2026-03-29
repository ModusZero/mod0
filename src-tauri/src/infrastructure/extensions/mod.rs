pub mod manifest;
pub mod runtime_proxy;
pub mod loader;

use std::sync::Arc;
use std::path::PathBuf;
use dashmap::DashMap;
use tauri::AppHandle;
use serde_json::Value;

use self::runtime_proxy::ExtensionRuntime;
use self::manifest::ExtensionManifest;
use crate::database::DbManager;
use crate::database::models::kernel::extension::Extension;

pub struct ExtensionHost {
    runtimes: DashMap<String, Arc<ExtensionRuntime>>,
    command_map: DashMap<String, String>,
    manifest_cache: DashMap<String, (ExtensionManifest, PathBuf)>,
    app: AppHandle,
    db: DbManager,
}

impl ExtensionHost {
    pub fn new(app: AppHandle, db: DbManager) -> Self {
        Self {
            runtimes: Arc::new(DashMap::new()),
            command_map: Arc::new(DashMap::new()),
            manifest_cache: Arc::new(DashMap::new()),
            app,
            db,
        }
    }

    pub async fn boot(&self, extensions: Vec<Extension>) {
        for ext in extensions {
            if !ext.is_enabled { continue; }
            
            if let Some(raw_manifest) = ext.manifest_cache {
                if let Ok(manifest) = serde_json::from_str::<ExtensionManifest>(&raw_manifest) {
                    let path = PathBuf::from(&ext.local_path);
                    self.register_internal(manifest, path);
                }
            }
        }
    }

    fn register_internal(&self, manifest: ExtensionManifest, path: PathBuf) {
        for cmd in &manifest.contributions.commands {
            self.command_map.insert(cmd.command.clone(), manifest.id.clone());
        }
        self.manifest_cache.insert(manifest.id.clone(), (manifest, path));
    }

    pub async fn dispatch(&self, command: &str, params: Value) -> Result<(), String> {
        let ext_id = match self.command_map.get(command) {
            Some(id) => id.clone(),
            None => return Ok(()), 
        };

        if !self.runtimes.contains_key(&ext_id) {
            self.activate(&ext_id).await?;
        }

        let runtime = self.runtimes.get(&ext_id)
            .ok_or_else(|| format!("Runtime lost for extension: {}", ext_id))?;
            
        runtime.send_request(command, params).await
    }

    /// Despacha un evento y espera una respuesta (ej. para transformaciones de texto)
    pub async fn dispatch_with_result(&self, command: &str, params: Value) -> Result<Value, String> {
        let ext_id = match self.command_map.get(command) {
            Some(id) => id.clone(),
            None => return Ok(Value::Null),
        };

        if !self.runtimes.contains_key(&ext_id) {
            self.activate(&ext_id).await?;
        }

        let runtime = self.runtimes.get(&ext_id)
            .ok_or_else(|| format!("Runtime lost for extension: {}", ext_id))?;
            
        // Nota: Asumimos que send_request en runtime_proxy puede retornar un Result<Value, String>
        // Si tu implementación actual de runtime_proxy no lo tiene, deberás ajustarlo para retornar el Result del JSON-RPC
        runtime.send_request_with_response(command, params).await
    }

    async fn activate(&self, id: &str) -> Result<(), String> {
        let (manifest, path) = self.manifest_cache.get(id)
            .ok_or_else(|| format!("Manifest not found in cache for: {}", id))?
            .clone();

        let rt = ExtensionRuntime::start(&manifest, path, self.app.clone()).await?;
        self.runtimes.insert(id.to_string(), Arc::new(rt));
        
        Ok(())
    }

    pub fn shutdown_extension(&self, id: &str) {
        self.runtimes.remove(id);
    }

    pub fn get_registered_commands(&self) -> Vec<String> {
        self.command_map.iter().map(|kv| kv.key().clone()).collect()
    }
}