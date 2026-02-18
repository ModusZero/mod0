use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use crate::services::bridge::providers::vcs::{VcsProvider, VcsCapabilities};
use crate::services::bridge::defs::ProviderConfig;

pub struct GithubProvider {
    config: ProviderConfig,
    client: Client,
}

impl GithubProvider {
    pub fn new(config: ProviderConfig) -> Self {
        Self { 
            config, 
            client: Client::builder().user_agent("Mod0-App").build().unwrap() 
        }
    }
}

#[async_trait]
impl VcsProvider for GithubProvider {
    fn capabilities(&self) -> VcsCapabilities {
        VcsCapabilities {
            can_pull_request: true,
            is_cloud: true,
        }
    }

    async fn execute_command(&self, _args: &[&str], _path: &str) -> Result<String, String> {
        Err("Para comandos git crudos en Cloud, usa el driver Local.".into())
    }

    async fn create_pull_request(&self, title: &str, head: &str, base: &str) -> Result<String, String> {
        let repo = self.config.settings["repo"].as_str().ok_or("Falta 'repo' en settings")?;
        let url = format!("https://api.github.com/repos/{}/pulls", repo);
        
        let res = self.client.post(url)
            .bearer_auth(&self.config.token)
            .json(&json!({ "title": title, "head": head, "base": base }))
            .send().await.map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
            Ok(data["html_url"].as_str().unwrap_or("PR Creado").to_string())
        } else {
            Err(res.text().await.unwrap_or_else(|_| "Error en GitHub API".into()))
        }
    }

    async fn check_health(&self) -> bool {
        let res = self.client.get("https://api.github.com/user")
            .bearer_auth(&self.config.token)
            .send().await;
        res.map(|r| r.status().is_success()).unwrap_or(false)
    }
}