use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleActionEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct BundleActionHandler {
    client: FilesClient,
}

impl BundleActionHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<BundleActionEntity>> {
        let response = self.client.get_raw("/bundle_actions").await?;
        let entities: Vec<BundleActionEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
