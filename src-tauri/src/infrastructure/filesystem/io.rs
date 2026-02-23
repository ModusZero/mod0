use std::fs;
use std::path::Path;

/// Operaciones puras de entrada/salida.
pub struct FileIO;

impl FileIO {
    pub fn read(path: &Path) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| e.to_string())
    }

    pub fn write(path: &Path, content: &str) -> Result<(), String> {
        fs::write(path, content).map_err(|e| e.to_string())
    }

    pub fn delete(path: &Path) -> Result<(), String> {
        if path.is_dir() {
            fs::remove_dir_all(path).map_err(|e| e.to_string())
        } else {
            fs::remove_file(path).map_err(|e| e.to_string())
        }
    }

    pub fn rename(old: &Path, new: &Path) -> Result<(), String> {
        fs::rename(old, new).map_err(|e| e.to_string())
    }

    pub fn create_dir(path: &Path) -> Result<(), String> {
        fs::create_dir_all(path).map_err(|e| e.to_string())
    }

    pub fn exists(path: &Path) -> bool {
        path.exists()
    }
}