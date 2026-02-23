use tokio::sync::RwLock;
use std::sync::Arc;
use crate::infrastructure::clients::defs::{ProviderConfig, VcsDriver};
use crate::infrastructure::clients::vcs::{
    VcsProvider, 
    github::GithubProvider, 
    gitlab::GitlabProvider,
    local::LocalGitProvider
};

/// Gestor de dominio para el control de versiones.
pub struct VcsManager {
    active_driver: RwLock<Option<Arc<dyn VcsProvider>>>,
}

impl VcsManager {
    pub fn new() -> Self {
        Self { active_driver: RwLock::new(None) }
    }

    pub async fn set_provider(&self, config: ProviderConfig) -> Result<(), String> {
        let driver_type = config.vcs_driver.as_ref().ok_or("No VCS driver type defined")?;

        let driver: Arc<dyn VcsProvider> = match driver_type {
            VcsDriver::Github => Arc::new(GithubProvider::new(config)),
            VcsDriver::Gitlab => Arc::new(GitlabProvider::new(config)),
            VcsDriver::GitLocal => Arc::new(LocalGitProvider::new(config)),
            _ => return Err("VCS Driver no implementado".into()),
        };

        let mut lock = self.active_driver.write().await;
        *lock = Some(driver);
        Ok(())
    }

    pub async fn execute(&self, args: &[&str], path: &str) -> Result<String, String> {
        let lock = self.active_driver.read().await;
        let driver = lock.as_ref().ok_or("VCS Provider no inicializado")?;
        driver.execute(args, path).await
    }

    pub async fn create_pr(&self, title: &str, head: &str, base: &str) -> Result<String, String> {
        let lock = self.active_driver.read().await;
        let driver = lock.as_ref().ok_or("VCS Provider no inicializado")?;
        driver.create_pr(title, head, base).await
    }
}