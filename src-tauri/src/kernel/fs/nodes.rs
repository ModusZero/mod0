use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Categoría de un nodo para definir su comportamiento en la interfaz y motor de búsqueda.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FileCategory {
    /// Archivos de código fuente indexables.
    Source,
    /// Archivos con prefijo punto.
    Hidden,
    /// Archivos excluidos por reglas de Git.
    Ignored,
    /// Archivos de sistema o configuración interna de la aplicación.
    System,
}

/// Estructura jerárquica que representa una entrada en el sistema de archivos.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileNode {
    /// Nombre base del archivo o carpeta.
    pub name: String,
    /// Ruta absoluta en el sistema de archivos.
    pub path: PathBuf,
    /// Define si el nodo es una carpeta.
    pub is_dir: bool,
    /// Clasificación basada en reglas de filtrado.
    pub category: FileCategory,
    /// Lista de nodos hijos en caso de ser un directorio.
    pub children: Option<Vec<FileNode>>,
}