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

/// Orquestador central de extensiones. 
/// Gestiona el ciclo de vida (Lazy Loading), el ruteo de comandos y el almacenamiento en caché.
pub struct ExtensionHost {
    /// Runtimes activos (procesos o conexiones abiertas)
    runtimes: DashMap<String, Arc<ExtensionRuntime>>,
    /// Mapa de ruteo rápido: "ID_COMANDO" -> "ID_EXTENSION"
    command_map: DashMap<String, String>,
    /// Caché de manifiestos y rutas para evitar hits constantes a la DB o Disco
    manifest_cache: DashMap<String, (ExtensionManifest, PathBuf)>,
    app: AppHandle,
    db: DbManager,
}

impl ExtensionHost {
    pub fn new(app: AppHandle, db: DbManager) -> Self {
        Self {
            runtimes: DashMap::new(),
            command_map: DashMap::new(),
            manifest_cache: DashMap::new(),
            app,
            db,
        }
    }

    /// Inicializa el Host cargando las extensiones habilitadas desde la base de datos.
    /// No inicia los procesos (Lazy), solo registra sus capacidades.
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

    /// Registro interno de comandos y manifiesto
    fn register_internal(&self, manifest: ExtensionManifest, path: PathBuf) {
        for cmd in &manifest.contributions.commands {
            self.command_map.insert(cmd.command.clone(), manifest.id.clone());
        }
        self.manifest_cache.insert(manifest.id.clone(), (manifest, path));
    }

    /// Envía un comando a la extensión correspondiente.
    /// Si la extensión no está activa, la inicializa automáticamente.
    pub async fn dispatch(&self, command: &str, params: Value) -> Result<(), String> {
        let ext_id = match self.command_map.get(command) {
            Some(id) => id.clone(),
            None => return Ok(()), // Comportamiento Middleware: Si nadie escucha, ignoramos
        };

        // Si no está en ejecución, activamos
        if !self.runtimes.contains_key(&ext_id) {
            self.activate(&ext_id).await?;
        }

        // Obtener runtime y despachar
        let runtime = self.runtimes.get(&ext_id)
            .ok_or_else(|| format!("Runtime lost for extension: {}", ext_id))?;
            
        runtime.send_request(command, params).await
    }

    /// Activa una extensión cargando su binario o estableciendo conexión según su protocolo.
    async fn activate(&self, id: &str) -> Result<(), String> {
        let (manifest, path) = self.manifest_cache.get(id)
            .ok_or_else(|| format!("Manifest not found in cache for: {}", id))?
            .clone();

        let rt = ExtensionRuntime::start(&manifest, path, self.app.clone()).await?;
        self.runtimes.insert(id.to_string(), Arc::new(rt));
        
        Ok(())
    }

    /// Apaga una extensión específica y libera sus recursos (pipes, sockets, etc.)
    pub fn shutdown_extension(&self, id: &str) {
        self.runtimes.remove(id);
    }

    /// Devuelve una lista de todos los comandos registrados actualmente en el sistema
    pub fn get_registered_commands(&self) -> Vec<String> {
        self.command_map.iter().map(|kv| kv.key().clone()).collect()
    }
}