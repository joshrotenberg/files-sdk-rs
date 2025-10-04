use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HolidayRegionEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct HolidayRegionHandler {
    client: FilesClient,
}

impl HolidayRegionHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<HolidayRegionEntity>> {
        let response = self.client.get_raw("/holiday_regions").await?;
        let entities: Vec<HolidayRegionEntity> = serde_json::from_value(response)?;
        Ok(entities)
    }
}
