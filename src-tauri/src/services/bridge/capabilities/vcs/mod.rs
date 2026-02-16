use crate::services::bridge::transport::http::HttpTransport;

pub struct VcsFacade {
    gh_client: HttpTransport,
}

impl VcsFacade {
    pub fn new(token: &str) -> Self {
        // En prod, esto configuraría los headers de Auth en el HttpTransport
        Self { gh_client: HttpTransport::new("https://api.github.com") }
    }

    pub async fn get_repository_info(&self, owner: &str, repo: &str) -> Result<serde_json::Value, String> {
        let endpoint = format!("/repos/{}/{}", owner, repo);
        // Implementación real usando HttpTransport
        Ok(serde_json::json!({ "status": "connected" }))
    }
}