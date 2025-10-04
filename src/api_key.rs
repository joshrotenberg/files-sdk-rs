use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiKeyCurrentEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct ApiKeyCurrentHandler {
    client: FilesClient,
}

impl ApiKeyCurrentHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn get_current(&self) -> Result<ApiKeyCurrentEntity> {
        let response = self.client.get_raw("/api_key").await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update(&self, params: serde_json::Value) -> Result<ApiKeyCurrentEntity> {
        let response = self.client.patch_raw("/api_key", params).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete(&self) -> Result<()> {
        self.client.delete_raw("/api_key").await?;
        Ok(())
    }
}
