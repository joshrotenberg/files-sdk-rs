use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionNotificationExportResultEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct ActionNotificationExportResultHandler {
    client: FilesClient,
}

impl ActionNotificationExportResultHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, id: i64) -> Result<ActionNotificationExportResultEntity> {
        let endpoint = format!("/action_notification_export_results/{}", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }
}
