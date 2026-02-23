use std::path::{Path, PathBuf};
use crate::kernel::fs::scanner::build_tree_recursive;
use crate::services::bridge::BridgeService;
use crate::services::bridge::defs::RuntimeDefinition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryResponse {
    pub summary: String,
    pub recommendations: Vec<RuntimeDefinition>,
}

pub struct DiscoveryEngine {
    project_root: PathBuf,
}

impl DiscoveryEngine {
    pub fn new(root: &Path) -> Self {
        Self {
            project_root: root.to_path_buf(),
        }
    }

    /// Orquesta el escaneo, consulta a la IA y devuelve las definiciones dinámicas.
    pub async fn discover_recommendations_for_workspace(&self, bridge: &BridgeService) -> Result<DiscoveryResponse, String> {
        // 1. Obtener el filetree (Contexto)
        let tree = build_tree_recursive(self.project_root.clone(), self.project_root.clone()).await?;
        let tree_json = serde_json::to_string(&tree).map_err(|e| e.to_string())?;

        // 2. Obtener lo que ya está instalado/configurado
        let installed = bridge.load_runtime_configs();
        let installed_json = serde_json::to_string(&installed).map_err(|e| e.to_string())?;

        // 3. Prompt para la IA especificando el formato RuntimeDefinition
        let prompt = format!(
            "Analiza este workspace y sugiere configuraciones de herramientas (LSP o MCP). 
            
            WORKSPACE:
            {}
            
            YA CONFIGURADO:
            {}

            Responde ÚNICAMENTE con un JSON que cumpla esta estructura:
            {{
                \"summary\": \"Breve explicación de qué detectaste\",
                \"recommendations\": [
                    {{
                        \"id\": \"string-unico\",
                        \"capability\": \"Lsp\" | \"Mcp\",
                        \"name\": \"Nombre Herramienta\",
                        \"command\": \"ejecutable\",
                        \"args\": [\"--arg\"],
                        \"strategy\": {{ \"type\": \"Lsp\" | \"Line\" | \"Raw\" }}
                    }}
                ]
            }}",
            tree_json, installed_json
        );

        // 4. Pedir recomendación a la IA
        let ai_response = bridge.ask_ai(&prompt).await?;

        // 5. Parsear y devolver al frontend
        let result: DiscoveryResponse = serde_json::from_str(&ai_response)
            .map_err(|e| format!("Error parseando recomendación de IA: {}", e))?;

        Ok(result)
    }
}