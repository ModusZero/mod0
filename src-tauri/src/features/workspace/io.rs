use std::fs;
use std::path::Path;

use crate::core::error::{AppError, AppResult};

/// Tamaño máximo que se abre en el editor.
const MAX_EDITABLE_BYTES: u64 = 5 * 1024 * 1024;

pub fn read_text(path: &Path) -> AppResult<String> {
    let size = fs::metadata(path)?.len();
    if size > MAX_EDITABLE_BYTES {
        return Err(AppError::msg(
            "el archivo supera los 5 MB; ábrelo con otra herramienta",
        ));
    }

    let bytes = fs::read(path)?;
    if bytes.iter().take(8192).any(|b| *b == 0) {
        return Err(AppError::msg("archivo binario: no se puede editar como texto"));
    }
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

pub fn write_text(path: &Path, content: &str) -> AppResult<()> {
    Ok(fs::write(path, content)?)
}

pub fn create_file(path: &Path) -> AppResult<()> {
    if path.exists() {
        return Err(AppError::msg("ya existe un elemento con ese nombre"));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(fs::write(path, "")?)
}

pub fn create_dir(path: &Path) -> AppResult<()> {
    Ok(fs::create_dir_all(path)?)
}

pub fn rename(from: &Path, to: &Path) -> AppResult<()> {
    if to.exists() {
        return Err(AppError::msg("ya existe un elemento con ese nombre"));
    }
    Ok(fs::rename(from, to)?)
}

pub fn delete(path: &Path) -> AppResult<()> {
    if path.is_dir() {
        Ok(fs::remove_dir_all(path)?)
    } else {
        Ok(fs::remove_file(path)?)
    }
}
