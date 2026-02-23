use super::{VcsProvider};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use crate::infrastructure::dispatchers::defs::{ProviderConfig, VcsCapabilities};

pub struct GitlabProvider {
    config: ProviderConfig,
    client: Client,
}

#[async_trait]
impl VcsProvider for GitlabProvider {
    fn new(config: ProviderConfig) -> Self {
        Self { 
            config, 
            client: Client::new() 
        }
    }

    fn capabilities(&self) -> VcsCapabilities {
        VcsCapabilities {
            can_pull_request: true, // En GitLab se llaman Merge Requests
            is_cloud: true,
        }
    }

    async fn execute(&self, _args: &[&str], _path: &str) -> Result<String, String> {
        Err("Usa driver Local para comandos git.".into())
    }

    async fn create_pr(&self, title: &str, head: &str, base: &str) -> Result<String, String> {
        // En GitLab el repo suele ser el ID numérico o el path url-encoded
        let project_id = self.config.settings["project_id"].as_str().ok_or("Falta 'project_id'")?;
        let base_url = self.config.endpoint.as_deref().unwrap_or("https://gitlab.com");
        let url = format!("{}/api/v4/projects/{}/merge_requests", base_url, project_id);

        let res = self.client.post(url)
            .header("PRIVATE-TOKEN", &self.config.token)
            .json(&json!({
                "title": title,
                "source_branch": head,
                "target_branch": base
            }))
            .send().await.map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
            Ok(data["web_url"].as_str().unwrap_or("MR Creado").to_string())
        } else {
            Err(res.text().await.unwrap_or_else(|_| "Error en GitLab API".into()))
        }
    }

    async fn check_health(&self) -> bool {
        let base_url = self.config.endpoint.as_deref().unwrap_or("https://gitlab.com");
        let res = self.client.get(format!("{}/api/v4/version", base_url))
            .header("PRIVATE-TOKEN", &self.config.token)
            .send().await;
        res.map(|r| r.status().is_success()).unwrap_or(false)
    }
}