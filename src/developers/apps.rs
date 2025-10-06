use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct AppHandler {
    client: FilesClient,
}

impl AppHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<AppEntity>> {
        let response = self.client.get_raw("/apps").await?;
        let entities: Vec<AppEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
