use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryExportEntity2 {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct HistoryExportHandler2 {
    client: FilesClient,
}

impl HistoryExportHandler2 {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, id: i64) -> Result<HistoryExportEntity2> {
        let endpoint = format!("/history_exports/{}", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn create(&self, params: serde_json::Value) -> Result<HistoryExportEntity2> {
        let response = self.client.post_raw("/history_exports", params).await?;
        Ok(serde_json::from_value(response)?)
    }
}
