use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileMigrationEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct FileMigrationHandler {
    client: FilesClient,
}

impl FileMigrationHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, id: i64) -> Result<FileMigrationEntity> {
        let endpoint = format!("/file_migrations/{}", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }
}
