use std::path::Path;
use std::process::{Command, Stdio};

use crate::core::error::{AppError, AppResult};

/// Crea un `Command` que no abre una ventana de consola en Windows.
pub fn command(program: &str) -> Command {
    #[allow(unused_mut)]
    let mut cmd = Command::new(program);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    cmd
}

/// Ejecuta un comando y devuelve su stdout (sin recortar). Si falla devuelve stderr.
pub fn run(program: &str, cwd: &Path, args: &[&str]) -> AppResult<Vec<u8>> {
    let output = command(program)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| AppError::msg(format!("no se pudo ejecutar `{program}`: {e}")))?;

    if output.status.success() {
        Ok(output.stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(AppError::Git(if stderr.is_empty() {
            format!("`{program} {}` terminó con {}", args.join(" "), output.status)
        } else {
            stderr
        }))
    }
}

/// Ejecuta trabajo bloqueante fuera del hilo principal de Tauri.
pub async fn blocking<T, F>(work: F) -> AppResult<T>
where
    F: FnOnce() -> AppResult<T> + Send + 'static,
    T: Send + 'static,
{
    tauri::async_runtime::spawn_blocking(work)
        .await
        .map_err(|e| AppError::msg(e.to_string()))?
}

/// Milisegundos desde epoch.
pub fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}
