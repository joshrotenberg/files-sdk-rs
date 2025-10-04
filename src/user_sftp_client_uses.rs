use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSftpClientUseEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct UserSftpClientUseHandler {
    client: FilesClient,
}

impl UserSftpClientUseHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<UserSftpClientUseEntity>> {
        let response = self.client.get_raw("/user_sftp_client_uses").await?;
        let entities: Vec<UserSftpClientUseEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
