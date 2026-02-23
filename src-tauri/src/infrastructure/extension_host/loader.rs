use tokio::fs;
use std::path::PathBuf;
use super::manifest::ExtensionManifest;

pub struct ExtensionLoader;

impl ExtensionLoader {
    pub async fn load_from_dir(dir: PathBuf) -> Option<(ExtensionManifest, PathBuf)> {
        let manifest_path = dir.join("manifest.json");
        let content = fs::read_to_string(manifest_path).await.ok()?;
        let manifest: ExtensionManifest = serde_json::from_str(&content).ok()?;
        Some((manifest, dir))
    }
}