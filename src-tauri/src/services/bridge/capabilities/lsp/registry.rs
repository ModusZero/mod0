use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransportType {
    Stdio,
    Wasm,
    // Próximamente: WebSockets, Http, etc.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LspDefinition {
    pub language_id: String,
    pub command: String,
    pub args: Vec<String>,
    pub transport: TransportType,
    pub extensions: Vec<String>, // Añadido para filtrar por extensión de archivo
}

impl LspDefinition {
    /// Devuelve la lista base de LSPs nativos de Mod0.
    /// En el futuro, esto podría leerse de un archivo JSON de configuración
    /// para permitir que los usuarios añadan sus propios LSPs sin recompilar.
    pub fn get_all_supported() -> Vec<Self> {
        vec![
            // --- SISTEMAS ---
            Self {
                language_id: "rust".into(),
                command: "rust-analyzer".into(),
                args: vec![],
                transport: TransportType::Stdio,
                extensions: vec!["rs".into()],
            },
            Self {
                language_id: "cpp".into(),
                command: "clangd".into(),
                args: vec!["--background-index".into()],
                transport: TransportType::Stdio,
                extensions: vec!["cpp".into(), "hpp".into(), "c".into(), "h".into()],
            },
            // --- WEB ---
            Self {
                language_id: "typescript".into(),
                command: "typescript-language-server".into(),
                args: vec!["--stdio".into()],
                transport: TransportType::Stdio,
                extensions: vec!["ts".into(), "tsx".into(), "js".into(), "jsx".into()],
            },
            Self {
                language_id: "svelte".into(),
                command: "svelte-language-server".into(),
                args: vec!["--stdio".into()],
                transport: TransportType::Stdio,
                extensions: vec!["svelte".into()],
            },
            // --- DATA / AI ---
            Self {
                language_id: "python".into(),
                command: "pyright-langserver".into(),
                args: vec!["--stdio".into()],
                transport: TransportType::Stdio,
                extensions: vec!["py".into()],
            },
        ]
    }

    pub fn get_definition(lang: &str) -> Option<Self> {
        Self::get_all_supported().into_iter().find(|d| d.language_id == lang)
    }

    /// Método clave para extensiones: Busca el LSP basado en la extensión del archivo
    pub fn get_by_extension(ext: &str) -> Option<Self> {
        Self::get_all_supported().into_iter()
            .find(|d| d.extensions.contains(&ext.to_string()))
    }
}