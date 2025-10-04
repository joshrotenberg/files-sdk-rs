//! SFTP host key management

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SftpHostKeyEntity {
    pub id: Option<i64>,
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct SftpHostKeyHandler {
    client: FilesClient,
}

impl SftpHostKeyHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        cursor: Option<String>,
        per_page: Option<i64>,
    ) -> Result<(Vec<SftpHostKeyEntity>, PaginationInfo)> {
        let mut endpoint = "/sftp_host_keys".to_string();
        let mut params = Vec::new();
        if let Some(c) = cursor {
            params.push(format!("cursor={}", c));
        }
        if let Some(pp) = per_page {
            params.push(format!("per_page={}", pp));
        }
        if !params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&params.join("&"));
        }

        let url = format!("{}{}", self.client.inner.base_url, endpoint);
        let response = reqwest::Client::new()
            .get(&url)
            .header("X-FilesAPI-Key", &self.client.inner.api_key)
            .send()
            .await?;

        let headers = response.headers().clone();
        let pagination = PaginationInfo::from_headers(&headers);
        let status = response.status();
        if !status.is_success() {
            return Err(crate::FilesError::ApiError {
                code: status.as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }
        let items: Vec<SftpHostKeyEntity> = response.json().await?;
        Ok((items, pagination))
    }

    pub async fn get(&self, id: i64) -> Result<SftpHostKeyEntity> {
        let endpoint = format!("/sftp_host_keys/{}", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn create(&self, params: serde_json::Value) -> Result<SftpHostKeyEntity> {
        let response = self.client.post_raw("/sftp_host_keys", params).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update(&self, id: i64, params: serde_json::Value) -> Result<SftpHostKeyEntity> {
        let endpoint = format!("/sftp_host_keys/{}", id);
        let response = self.client.patch_raw(&endpoint, params).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        let endpoint = format!("/sftp_host_keys/{}", id);
        self.client.delete_raw(&endpoint).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = SftpHostKeyHandler::new(client);
    }
}
