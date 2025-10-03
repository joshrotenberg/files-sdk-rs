//! API Key management operations
//!
//! This module provides API key management functionality including:
//! - List API keys
//! - Create new API keys
//! - Update API key details
//! - Delete API keys
//!
//! API keys are the recommended authentication method for programmatic access.

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// API Key entity from Files.com API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyEntity {
    /// API Key ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// API Key name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// API Key description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Descriptive label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_label: Option<String>,

    /// The actual API key (only returned on creation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// User ID this key belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// Platform this key is for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,

    /// Permission set for this key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_set: Option<String>,

    /// URL this key is associated with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Created at timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Expires at timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    /// Last use at timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_use_at: Option<String>,
}

/// Handler for API key operations
#[derive(Debug, Clone)]
pub struct ApiKeyHandler {
    client: FilesClient,
}

impl ApiKeyHandler {
    /// Creates a new ApiKeyHandler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List API keys
    ///
    /// # Arguments
    ///
    /// * `user_id` - Filter by user ID (optional)
    /// * `cursor` - Pagination cursor (optional)
    /// * `per_page` - Results per page (optional)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, ApiKeyHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = ApiKeyHandler::new(client);
    /// let (keys, pagination) = handler.list(None, None, Some(10)).await?;
    ///
    /// for key in keys {
    ///     println!("API Key: {:?}", key.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        user_id: Option<i64>,
        cursor: Option<String>,
        per_page: Option<i32>,
    ) -> Result<(Vec<ApiKeyEntity>, PaginationInfo)> {
        let mut path = "/api_keys?".to_string();

        if let Some(uid) = user_id {
            path.push_str(&format!("user_id={}&", uid));
        }
        if let Some(c) = cursor {
            path.push_str(&format!("cursor={}&", c));
        }
        if let Some(pp) = per_page {
            path.push_str(&format!("per_page={}&", pp));
        }

        let response = self.client.get_raw(&path).await?;
        let keys: Vec<ApiKeyEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((keys, pagination))
    }

    /// Get a specific API key by ID
    ///
    /// # Arguments
    ///
    /// * `id` - API Key ID
    pub async fn get(&self, id: i64) -> Result<ApiKeyEntity> {
        let path = format!("/api_keys/{}", id);
        let response = self.client.get_raw(&path).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new API key
    ///
    /// # Arguments
    ///
    /// * `name` - Name for the API key (optional)
    /// * `description` - Description (optional)
    /// * `expires_at` - Expiration timestamp (optional)
    /// * `permission_set` - Permission set name (optional)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, ApiKeyHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = ApiKeyHandler::new(client);
    /// let key = handler.create(
    ///     Some("My API Key"),
    ///     Some("For automated uploads"),
    ///     None,
    ///     None
    /// ).await?;
    /// println!("Created key: {:?}", key.key);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        name: Option<&str>,
        description: Option<&str>,
        expires_at: Option<&str>,
        permission_set: Option<&str>,
    ) -> Result<ApiKeyEntity> {
        let mut body = json!({});

        if let Some(n) = name {
            body["name"] = json!(n);
        }
        if let Some(d) = description {
            body["description"] = json!(d);
        }
        if let Some(e) = expires_at {
            body["expires_at"] = json!(e);
        }
        if let Some(p) = permission_set {
            body["permission_set"] = json!(p);
        }

        let response = self.client.post_raw("/api_keys", body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update an API key
    ///
    /// # Arguments
    ///
    /// * `id` - API Key ID
    /// * `name` - New name (optional)
    /// * `description` - New description (optional)
    /// * `expires_at` - New expiration (optional)
    pub async fn update(
        &self,
        id: i64,
        name: Option<&str>,
        description: Option<&str>,
        expires_at: Option<&str>,
    ) -> Result<ApiKeyEntity> {
        let mut body = json!({});

        if let Some(n) = name {
            body["name"] = json!(n);
        }
        if let Some(d) = description {
            body["description"] = json!(d);
        }
        if let Some(e) = expires_at {
            body["expires_at"] = json!(e);
        }

        let path = format!("/api_keys/{}", id);
        let response = self.client.patch_raw(&path, body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete an API key
    ///
    /// # Arguments
    ///
    /// * `id` - API Key ID
    pub async fn delete(&self, id: i64) -> Result<()> {
        let path = format!("/api_keys/{}", id);
        self.client.delete_raw(&path).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = ApiKeyHandler::new(client);
    }
}
