//! Group user membership operations

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Represents a user's membership in a group
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupUserEntity {
    pub id: Option<i64>,
    pub group_name: Option<String>,
    pub group_id: Option<i64>,
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub admin: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct GroupUserHandler {
    client: FilesClient,
}

impl GroupUserHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        user_id: Option<i64>,
        cursor: Option<String>,
        per_page: Option<i64>,
    ) -> Result<(Vec<GroupUserEntity>, PaginationInfo)> {
        let mut endpoint = "/group_users".to_string();
        let mut params = Vec::new();

        if let Some(uid) = user_id {
            params.push(format!("user_id={}", uid));
        }
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
            return Err(crate::FilesError::ApiError { endpoint: None,
                code: status.as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let users: Vec<GroupUserEntity> = response.json().await?;
        Ok((users, pagination))
    }

    pub async fn create(
        &self,
        group_id: i64,
        user_id: i64,
        admin: bool,
    ) -> Result<GroupUserEntity> {
        let body = json!({
            "group_id": group_id,
            "user_id": user_id,
            "admin": admin,
        });

        let response = self.client.post_raw("/group_users", body).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn update(&self, id: i64, admin: bool) -> Result<GroupUserEntity> {
        let body = json!({"admin": admin});
        let endpoint = format!("/group_users/{}", id);
        let response = self.client.patch_raw(&endpoint, body).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        let endpoint = format!("/group_users/{}", id);
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
        let _handler = GroupUserHandler::new(client);
    }
}
