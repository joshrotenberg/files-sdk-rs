//! Snapshot management

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapshotEntity {
    pub id: Option<i64>,
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct SnapshotHandler {
    client: FilesClient,
}

impl SnapshotHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        cursor: Option<String>,
        per_page: Option<i64>,
    ) -> Result<(Vec<SnapshotEntity>, PaginationInfo)> {
        let mut endpoint = "/snapshots".to_string();
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
        let items: Vec<SnapshotEntity> = response.json().await?;
        Ok((items, pagination))
    }

    pub async fn get(&self, id: i64) -> Result<SnapshotEntity> {
        let endpoint = format!("/snapshots/{}", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn create(&self, params: serde_json::Value) -> Result<SnapshotEntity> {
        let response = self.client.post_raw("/snapshots", params).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update(&self, id: i64, params: serde_json::Value) -> Result<SnapshotEntity> {
        let endpoint = format!("/snapshots/{}", id);
        let response = self.client.patch_raw(&endpoint, params).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        let endpoint = format!("/snapshots/{}", id);
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
        let _handler = SnapshotHandler::new(client);
    }
}
