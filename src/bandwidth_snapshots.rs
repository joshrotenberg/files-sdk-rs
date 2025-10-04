use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BandwidthSnapshotEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct BandwidthSnapshotHandler {
    client: FilesClient,
}

impl BandwidthSnapshotHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<BandwidthSnapshotEntity>> {
        let response = self.client.get_raw("/bandwidth_snapshots").await?;
        let entities: Vec<BandwidthSnapshotEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
