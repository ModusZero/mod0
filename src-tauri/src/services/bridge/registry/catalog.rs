use crate::services::bridge::registry::defs::RuntimeDefinition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCatalog {
    /// Mapea extensiones a IDs de definiciones
    pub extensions: HashMap<String, String>,
    /// Mapea archivos exactos (package.json, Cargo.toml, requirements.txt) a IDs de definiciones
    pub signatures: HashMap<String, String>,
    /// El diccionario de herramientas disponibles
    pub definitions: HashMap<String, RuntimeDefinition>,
}

impl ToolCatalog {
    /// Carga el catálogo desde el archivo TOML embebido
    pub fn load_internal() -> Self {
        let data = include_str!("catalog.toml");
        
        match tomllib::from_str::<ToolCatalog>(data) {
            Ok(catalog) => catalog,
            Err(e) => {
                panic!("Error crítico en catalog.toml: {}", e);
            }
        }
    }

    /// Método de seguridad: Valida que todos los IDs en extensions/signatures existan en definitions
    pub fn validate(&self) -> Result<(), String> {
        for (ext, id) in &self.extensions {
            if !self.definitions.contains_key(id) {
                return Err(format!("Extensión '{}' apunta a ID inexistente '{}'", ext, id));
            }
        }
        for (sig, id) in &self.signatures {
            if !self.definitions.contains_key(id) {
                return Err(format!("Firma '{}' apunta a ID inexistente '{}'", sig, id));
            }
        }
        Ok(())
    }
}