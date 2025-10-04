use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct As2IncomingMessageEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct As2IncomingMessageHandler {
    client: FilesClient,
}

impl As2IncomingMessageHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<As2IncomingMessageEntity>> {
        let response = self.client.get_raw("/as2_incoming_messages").await?;
        let entities: Vec<As2IncomingMessageEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
