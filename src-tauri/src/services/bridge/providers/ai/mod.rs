use async_trait::async_trait;

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn completion(&self, prompt: &str) -> Result<String, String>;
}

pub mod drivers;