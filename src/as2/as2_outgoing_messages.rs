use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct As2OutgoingMessageEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct As2OutgoingMessageHandler {
    client: FilesClient,
}

impl As2OutgoingMessageHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<As2OutgoingMessageEntity>> {
        let response = self.client.get_raw("/as2_outgoing_messages").await?;
        let entities: Vec<As2OutgoingMessageEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
