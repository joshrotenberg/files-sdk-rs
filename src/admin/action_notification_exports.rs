use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionNotificationExportEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct ActionNotificationExportHandler {
    client: FilesClient,
}

impl ActionNotificationExportHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, id: i64) -> Result<ActionNotificationExportEntity> {
        let endpoint = format!("/action_notification_exports/{}", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn create(
        &self,
        params: serde_json::Value,
    ) -> Result<ActionNotificationExportEntity> {
        let response = self
            .client
            .post_raw("/action_notification_exports", params)
            .await?;
        Ok(serde_json::from_value(response)?)
    }
}
