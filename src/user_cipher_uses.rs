use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCipherUseEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct UserCipherUseHandler {
    client: FilesClient,
}

impl UserCipherUseHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<UserCipherUseEntity>> {
        let response = self.client.get_raw("/user_cipher_uses").await?;
        let entities: Vec<UserCipherUseEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
