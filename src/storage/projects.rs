//! Project management operations

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Represents a project
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectEntity {
    pub id: Option<i64>,
    pub global_access: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProjectHandler {
    client: FilesClient,
}

impl ProjectHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        cursor: Option<String>,
        per_page: Option<i64>,
    ) -> Result<(Vec<ProjectEntity>, PaginationInfo)> {
        let mut endpoint = "/projects".to_string();
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
                endpoint: None,
                code: status.as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let projects: Vec<ProjectEntity> = response.json().await?;
        Ok((projects, pagination))
    }

    pub async fn get(&self, id: i64) -> Result<ProjectEntity> {
        let endpoint = format!("/projects/{}", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn create(&self, global_access: &str) -> Result<ProjectEntity> {
        let body = json!({"global_access": global_access});
        let response = self.client.post_raw("/projects", body).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update(&self, id: i64, global_access: &str) -> Result<ProjectEntity> {
        let body = json!({"global_access": global_access});
        let endpoint = format!("/projects/{}", id);
        let response = self.client.patch_raw(&endpoint, body).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        let endpoint = format!("/projects/{}", id);
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
        let _handler = ProjectHandler::new(client);
    }
}
