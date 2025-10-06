//! API Key management operations
//!
//! Provides secure API key management for programmatic access to Files.com.
//! API keys are the recommended authentication method for applications and scripts.
//!
//! # Features
//!
//! - Create and revoke API keys
//! - Set expiration dates
//! - Configure permission sets
//! - Track key usage and last access
//! - Manage keys for specific users
//!
//! # Security Best Practices
//!
//! - Store API keys securely (environment variables, secret managers)
//! - Set expiration dates for temporary access
//! - Use permission sets to limit key capabilities
//! - Rotate keys periodically
//! - Delete unused keys immediately
//!
//! # Example
//!
//! ```no_run
//! use files_sdk::{FilesClient, ApiKeyHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = FilesClient::builder()
//!     .api_key("your-api-key")
//!     .build()?;
//!
//! let handler = ApiKeyHandler::new(client);
//!
//! // Create a new API key with expiration
//! let key = handler.create(
//!     Some("Automation Script Key"),
//!     Some("For nightly backup automation"),
//!     Some("2025-12-31T23:59:59Z"),
//!     None
//! ).await?;
//!
//! // IMPORTANT: Save this key securely - it won't be shown again
//! println!("New API Key: {}", key.key.unwrap());
//! println!("Key ID: {}", key.id.unwrap());
//!
//! // List all API keys
//! let (keys, _) = handler.list(None, None, Some(50)).await?;
//! for api_key in keys {
//!     println!("{}: Last used {}",
//!         api_key.name.unwrap_or_default(),
//!         api_key.last_use_at.unwrap_or_default());
//! }
//! # Ok(())
//! # }
//! ```

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
    /// Returns a paginated list of API keys with usage information and metadata.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Filter by specific user ID (None for all users)
    /// * `cursor` - Pagination cursor from previous response
    /// * `per_page` - Number of results per page (max 10,000)
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - Vector of `ApiKeyEntity` objects
    /// - `PaginationInfo` with cursors for next/previous pages
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, ApiKeyHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = ApiKeyHandler::new(client);
    ///
    /// // List all API keys
    /// let (keys, pagination) = handler.list(None, None, Some(50)).await?;
    ///
    /// for key in keys {
    ///     println!("{}: Created {} - Last used {}",
    ///         key.name.unwrap_or_default(),
    ///         key.created_at.unwrap_or_default(),
    ///         key.last_use_at.unwrap_or_default());
    /// }
    ///
    /// // Get keys for specific user
    /// let (user_keys, _) = handler.list(Some(12345), None, None).await?;
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

    /// Get details of a specific API key
    ///
    /// # Arguments
    ///
    /// * `id` - API Key ID
    ///
    /// # Returns
    ///
    /// An `ApiKeyEntity` with key details (note: the actual key value is not included)
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, ApiKeyHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = ApiKeyHandler::new(client);
    ///
    /// let key = handler.get(12345).await?;
    /// println!("Key name: {}", key.name.unwrap_or_default());
    /// println!("Expires: {}", key.expires_at.unwrap_or_default());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, id: i64) -> Result<ApiKeyEntity> {
        let path = format!("/api_keys/{}", id);
        let response = self.client.get_raw(&path).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new API key
    ///
    /// Generates a new API key for programmatic access. The key value is only
    /// returned once during creation - save it securely immediately.
    ///
    /// # Arguments
    ///
    /// * `name` - Descriptive name for the key
    /// * `description` - Detailed description of key purpose
    /// * `expires_at` - ISO 8601 expiration timestamp (e.g., "2025-12-31T23:59:59Z")
    /// * `permission_set` - Named permission set to limit key capabilities
    ///
    /// # Returns
    ///
    /// An `ApiKeyEntity` with the `key` field containing the actual API key.
    /// This is the only time the key value will be visible.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, ApiKeyHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = ApiKeyHandler::new(client);
    ///
    /// // Create key with expiration
    /// let key = handler.create(
    ///     Some("CI/CD Pipeline Key"),
    ///     Some("For automated deployments"),
    ///     Some("2025-12-31T23:59:59Z"),
    ///     None
    /// ).await?;
    ///
    /// // CRITICAL: Save this key now - it won't be shown again!
    /// println!("API Key: {}", key.key.unwrap());
    /// println!("Store this securely in your CI/CD secrets");
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

    /// Update an API key's metadata
    ///
    /// Modifies key information such as name, description, or expiration.
    /// The actual key value cannot be changed.
    ///
    /// # Arguments
    ///
    /// * `id` - API Key ID to update
    /// * `name` - New descriptive name
    /// * `description` - New description
    /// * `expires_at` - New expiration timestamp
    ///
    /// # Returns
    ///
    /// The updated `ApiKeyEntity`
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, ApiKeyHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = ApiKeyHandler::new(client);
    ///
    /// // Extend expiration date
    /// let key = handler.update(
    ///     12345,
    ///     None,
    ///     None,
    ///     Some("2026-12-31T23:59:59Z")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Delete an API key permanently
    ///
    /// Revokes the API key immediately. Any requests using this key will fail.
    /// This operation cannot be undone.
    ///
    /// # Arguments
    ///
    /// * `id` - API Key ID to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, ApiKeyHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = ApiKeyHandler::new(client);
    ///
    /// // Revoke compromised or unused key
    /// handler.delete(12345).await?;
    /// println!("API key revoked successfully");
    /// # Ok(())
    /// # }
    /// ```
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
