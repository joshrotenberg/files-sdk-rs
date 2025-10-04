use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsageSnapshotEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct UsageSnapshotHandler {
    client: FilesClient,
}

impl UsageSnapshotHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<UsageSnapshotEntity>> {
        let response = self.client.get_raw("/usage_snapshots").await?;
        let entities: Vec<UsageSnapshotEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
