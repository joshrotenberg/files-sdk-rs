use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleDownloadEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct BundleDownloadHandler {
    client: FilesClient,
}

impl BundleDownloadHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<BundleDownloadEntity>> {
        let response = self.client.get_raw("/bundle_downloads").await?;
        let entities: Vec<BundleDownloadEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
