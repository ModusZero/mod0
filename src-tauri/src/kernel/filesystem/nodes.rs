use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Define la categoría de un archivo para determinar su tratamiento en la UI y el motor de búsqueda.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FileCategory {
    /// Archivos de código fuente que deben ser indexados por contenido.
    Source,
    /// Archivos u ocultos (con nombres comenzados con . como .git, .env).
    Hidden,
    /// Archivos omitidos por el sistema de control de versiones (.gitignore).
    Ignored,
    /// Archivos omitidos por el nuevo sistema de .scanignore para MOD0.
    System,
}

/// Representa un nodo (archivo o carpeta) en el sistema de archivos.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileNode {
    /// Nombre del archivo o carpeta.
    pub name: String,
    /// Ruta absoluta en el sistema.
    pub path: PathBuf,
    /// Indica si el nodo es un directorio.
    pub is_dir: bool,
    /// Categoría asignada basada en reglas de filtrado jerárquico.
    pub category: FileCategory,
    /// Hijos del nodo si es un directorio.
    pub children: Option<Vec<FileNode>>,
}