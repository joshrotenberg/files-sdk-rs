use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailIncomingMessageEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct EmailIncomingMessageHandler {
    client: FilesClient,
}

impl EmailIncomingMessageHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<EmailIncomingMessageEntity>> {
        let response = self.client.get_raw("/email_incoming_messages").await?;
        let entities: Vec<EmailIncomingMessageEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
