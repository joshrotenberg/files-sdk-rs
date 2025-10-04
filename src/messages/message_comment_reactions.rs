use crate::{Result, client::FilesClient, types::PaginationInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageCommentReactionEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct MessageCommentReactionHandler {
    client: FilesClient,
}

impl MessageCommentReactionHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        message_comment_id: i64,
        cursor: Option<String>,
        per_page: Option<i64>,
    ) -> Result<(Vec<MessageCommentReactionEntity>, PaginationInfo)> {
        let mut endpoint = "/message_comment_reactions".to_string();
        let mut query_params = vec![format!("message_comment_id={}", message_comment_id)];

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

        let entities: Vec<MessageCommentReactionEntity> = response.json().await?;
        Ok((entities, pagination))
    }

    pub async fn get(&self, id: i64) -> Result<MessageCommentReactionEntity> {
        let endpoint = format!("/message_comment_reactions/{}", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn create(&self, params: serde_json::Value) -> Result<MessageCommentReactionEntity> {
        let response = self
            .client
            .post_raw("/message_comment_reactions", params)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        let endpoint = format!("/message_comment_reactions/{}", id);
        self.client.delete_raw(&endpoint).await?;
        Ok(())
    }
}
