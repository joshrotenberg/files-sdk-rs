use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsageDailySnapshotEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct UsageDailySnapshotHandler {
    client: FilesClient,
}

impl UsageDailySnapshotHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<UsageDailySnapshotEntity>> {
        let response = self.client.get_raw("/usage_daily_snapshots").await?;
        let entities: Vec<UsageDailySnapshotEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
