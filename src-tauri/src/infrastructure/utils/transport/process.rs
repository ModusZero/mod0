use tokio::process::Command;
use std::process::Stdio;

pub struct ProcessManager;

impl ProcessManager {
    /// Spawnea un comando y devuelve sus pipes
    pub fn spawn_with_pipes(cmd: &str, args: &[&str]) -> Result<tokio::process::Child, String> {
        Command::new(cmd)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Fallo al iniciar proceso {}: {}", cmd, e))
    }
}