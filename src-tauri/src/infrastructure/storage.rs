use std::fs;
use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::core::error::AppResult;

/// Lee un JSON tipado. Devuelve `None` si el archivo no existe.
pub fn read_json<T: DeserializeOwned>(path: &Path) -> AppResult<Option<T>> {
    if !path.exists() {
        return Ok(None);
    }

    let raw = fs::read_to_string(path)?;
    Ok(Some(serde_json::from_str(&raw)?))
}

/// Escribe un JSON de forma atómica (archivo temporal + rename) para no corromper
/// el estado si la app se cierra a mitad de la escritura.
pub fn write_json_atomic<T: Serialize + ?Sized>(path: &Path, value: &T) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_vec_pretty(value)?)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

/// Hash FNV-1a estable (no criptográfico) para derivar nombres de archivo de una ruta.
pub fn stable_key(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}
