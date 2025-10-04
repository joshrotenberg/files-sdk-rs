use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct CurrentUserHandler {
    client: FilesClient,
}

impl CurrentUserHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn get_current(&self) -> Result<UserEntity> {
        let response = self.client.get_raw("/user").await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update(&self, params: serde_json::Value) -> Result<UserEntity> {
        let response = self.client.patch_raw("/user", params).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn unlock(&self, id: i64) -> Result<()> {
        let endpoint = format!("/users/{}/unlock", id);
        self.client.post_raw(&endpoint, json!({})).await?;
        Ok(())
    }

    pub async fn resend_verification(&self, id: i64) -> Result<()> {
        let endpoint = format!("/users/{}/resend_verification_email", id);
        self.client.post_raw(&endpoint, json!({})).await?;
        Ok(())
    }

    pub async fn get_2fa_status(&self, id: i64) -> Result<UserEntity> {
        let endpoint = format!("/users/{}/2fa", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update_2fa(&self, id: i64, params: serde_json::Value) -> Result<UserEntity> {
        let endpoint = format!("/users/{}/2fa", id);
        let response = self.client.patch_raw(&endpoint, params).await?;
        Ok(serde_json::from_value(response)?)
    }
}
