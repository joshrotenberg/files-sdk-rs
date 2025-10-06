use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StyleEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct StyleHandler {
    client: FilesClient,
}

impl StyleHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, path: &str) -> Result<StyleEntity> {
        let endpoint = format!("/styles/{}", path);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update(&self, path: &str, params: serde_json::Value) -> Result<StyleEntity> {
        let endpoint = format!("/styles/{}", path);
        let response = self.client.patch_raw(&endpoint, params).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let endpoint = format!("/styles/{}", path);
        self.client.delete_raw(&endpoint).await?;
        Ok(())
    }
}
