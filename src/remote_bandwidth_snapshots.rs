use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteBandwidthSnapshotEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct RemoteBandwidthSnapshotHandler {
    client: FilesClient,
}

impl RemoteBandwidthSnapshotHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<RemoteBandwidthSnapshotEntity>> {
        let response = self.client.get_raw("/remote_bandwidth_snapshots").await?;
        let entities: Vec<RemoteBandwidthSnapshotEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
