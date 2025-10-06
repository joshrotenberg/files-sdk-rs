//! API request logging

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiRequestLogEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct ApiRequestLogHandler {
    client: FilesClient,
}

impl ApiRequestLogHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        cursor: Option<String>,
        per_page: Option<i64>,
    ) -> Result<(Vec<ApiRequestLogEntity>, PaginationInfo)> {
        let mut endpoint = "/api_request_logs".to_string();
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

        let logs: Vec<ApiRequestLogEntity> = response.json().await?;
        Ok((logs, pagination))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = ApiRequestLogHandler::new(client);
    }
}
