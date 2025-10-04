use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicHostingRequestLogEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct PublicHostingRequestLogHandler {
    client: FilesClient,
}

impl PublicHostingRequestLogHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<PublicHostingRequestLogEntity>> {
        let response = self.client.get_raw("/public_hosting_request_logs").await?;
        Ok(serde_json::from_value(response)?)
    }
}
