use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InboxRegistrationEntity2 {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct InboxRegistrationHandler2 {
    client: FilesClient,
}

impl InboxRegistrationHandler2 {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<InboxRegistrationEntity2>> {
        let response = self.client.get_raw("/inbox_registrations").await?;
        let entities: Vec<InboxRegistrationEntity2> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
