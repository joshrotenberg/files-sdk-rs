use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnsRecordEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct DnsRecordHandler {
    client: FilesClient,
}

impl DnsRecordHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<DnsRecordEntity>> {
        let response = self.client.get_raw("/dns_records").await?;
        let entities: Vec<DnsRecordEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
