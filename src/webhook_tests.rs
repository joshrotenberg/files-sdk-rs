use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookTestEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct WebhookTestHandler {
    client: FilesClient,
}

impl WebhookTestHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn create(&self, params: serde_json::Value) -> Result<WebhookTestEntity> {
        let response = self.client.post_raw("/webhook_tests", params).await?;
        let entity: WebhookTestEntity = serde_json::from_value(response)?;
        Ok(entity)
    }
}
