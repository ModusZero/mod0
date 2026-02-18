use async_trait::async_trait;
use std::process::Command;
use crate::services::bridge::providers::vcs::{VcsProvider, VcsCapabilities};
use crate::services::bridge::defs::ProviderConfig;

pub struct LocalGitProvider {
    _config: ProviderConfig,
}

impl LocalGitProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self { _config: config }
    }
}

#[async_trait]
impl VcsProvider for LocalGitProvider {
    fn capabilities(&self) -> VcsCapabilities {
        VcsCapabilities {
            can_pull_request: false,
            is_cloud: false,
        }
    }

    async fn execute_command(&self, args: &[&str], path: &str) -> Result<String, String> {
        let output = Command::new("git")
            .current_dir(path)
            .args(args)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    async fn create_pull_request(&self, _: &str, _: &str, _: &str) -> Result<String, String> {
        Err("Git Local no soporta Pull Requests.".into())
    }

    async fn check_health(&self) -> bool {
        // Verifica si git está instalado
        Command::new("git").arg("--version").status().is_ok()
    }
}