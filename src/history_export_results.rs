use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryExportResultEntity2 {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct HistoryExportResultHandler2 {
    client: FilesClient,
}

impl HistoryExportResultHandler2 {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, id: i64) -> Result<HistoryExportResultEntity2> {
        let endpoint = format!("/history_export_results/{}", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }
}
