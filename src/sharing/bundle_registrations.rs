use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleRegistrationEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct BundleRegistrationHandler {
    client: FilesClient,
}

impl BundleRegistrationHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<BundleRegistrationEntity>> {
        let response = self.client.get_raw("/bundle_registrations").await?;
        let entities: Vec<BundleRegistrationEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
