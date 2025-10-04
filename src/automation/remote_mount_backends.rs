use crate::{Result, client::FilesClient, types::PaginationInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteMountBackendEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct RemoteMountBackendHandler {
    client: FilesClient,
}

impl RemoteMountBackendHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        cursor: Option<String>,
        per_page: Option<i64>,
    ) -> Result<(Vec<RemoteMountBackendEntity>, PaginationInfo)> {
        let mut endpoint = "/remote_mount_backends".to_string();
        let mut query_params = Vec::new();

        if let Some(cursor) = cursor {
            query_params.push(format!("cursor={}", cursor));
        }

        if let Some(per_page) = per_page {
            query_params.push(format!("per_page={}", per_page));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
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

        let entities: Vec<RemoteMountBackendEntity> = response.json().await?;
        Ok((entities, pagination))
    }

    pub async fn get(&self, id: i64) -> Result<RemoteMountBackendEntity> {
        let endpoint = format!("/remote_mount_backends/{}", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn create(&self, params: serde_json::Value) -> Result<RemoteMountBackendEntity> {
        let response = self
            .client
            .post_raw("/remote_mount_backends", params)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update(
        &self,
        id: i64,
        params: serde_json::Value,
    ) -> Result<RemoteMountBackendEntity> {
        let endpoint = format!("/remote_mount_backends/{}", id);
        let response = self.client.patch_raw(&endpoint, params).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        let endpoint = format!("/remote_mount_backends/{}", id);
        self.client.delete_raw(&endpoint).await?;
        Ok(())
    }

    pub async fn test_configuration(
        &self,
        id: i64,
        params: serde_json::Value,
    ) -> Result<RemoteMountBackendEntity> {
        let endpoint = format!("/remote_mount_backends/{}/test_configuration", id);
        let response = self.client.post_raw(&endpoint, params).await?;
        Ok(serde_json::from_value(response)?)
    }
}
