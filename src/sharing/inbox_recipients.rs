use crate::{Result, client::FilesClient, types::PaginationInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InboxRecipientEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct InboxRecipientHandler {
    client: FilesClient,
}

impl InboxRecipientHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        inbox_id: i64,
        cursor: Option<String>,
        per_page: Option<i64>,
    ) -> Result<(Vec<InboxRecipientEntity>, PaginationInfo)> {
        let mut endpoint = "/inbox_recipients".to_string();
        let mut query_params = vec![format!("inbox_id={}", inbox_id)];

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
                endpoint: None,
                code: status.as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let entities: Vec<InboxRecipientEntity> = response.json().await?;
        Ok((entities, pagination))
    }

    pub async fn create(&self, params: serde_json::Value) -> Result<InboxRecipientEntity> {
        let response = self.client.post_raw("/inbox_recipients", params).await?;
        Ok(serde_json::from_value(response)?)
    }
}
