// pub mod providers;
// use serde::{Deserialize, Serialize};
// use crate::services::bridge::transport::http::HttpTransport;

// #[derive(Serialize, Deserialize)]
// pub struct AiMessage { pub role: String, pub content: String }

// pub struct AiFacade {
//     client: HttpTransport,
// }

// impl AiFacade {
//     pub fn new(base_url: &str) -> Self {
//         Self { client: HttpTransport::new(base_url) }
//     }

//     pub async fn chat(&self, provider: &str, messages: Vec<AiMessage>) -> Result<String, String> {
//         match provider {
//             "ollama" => providers::ollama::request(&self.client, messages).await,
//             "claude" => providers::anthropic::request(&self.client, messages).await,
//             _ => Err("Proveedor no implementado".into())
//         }
//     }
// }