use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExavaultApiRequestLogEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct ExavaultApiRequestLogHandler {
    client: FilesClient,
}

impl ExavaultApiRequestLogHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<ExavaultApiRequestLogEntity>> {
        let response = self.client.get_raw("/exavault_api_request_logs").await?;
        let entities: Vec<ExavaultApiRequestLogEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
