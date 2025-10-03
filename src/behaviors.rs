//! Behavior operations
//!
//! Behaviors are folder-level settings that automate actions like webhooks,
//! file expiration, encryption, and more.

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// A Behavior entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorEntity {
    /// Behavior ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Folder path where behavior applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// URL for attached file (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_url: Option<String>,

    /// Behavior type (e.g., webhook, auto_encrypt, file_expiration)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<String>,

    /// Description of this behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Name of the behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Behavior configuration value (hash/object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,

    /// Disable parent folder behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_parent_folder_behavior: Option<bool>,

    /// Apply recursively to subfolders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

/// Handler for behavior operations
pub struct BehaviorHandler {
    client: FilesClient,
}

impl BehaviorHandler {
    /// Create a new behavior handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List behaviors
    ///
    /// # Arguments
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    /// * `sort_by` - Sort field and direction
    /// * `filter` - Filter criteria
    /// * `filter_prefix` - Filter by path prefix
    ///
    /// # Returns
    /// Tuple of (behaviors, pagination_info)
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, BehaviorHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = BehaviorHandler::new(client);
    /// let (behaviors, _) = handler.list(None, None, None, None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        &self,
        cursor: Option<&str>,
        per_page: Option<i64>,
        sort_by: Option<serde_json::Value>,
        filter: Option<serde_json::Value>,
        filter_prefix: Option<&str>,
    ) -> Result<(Vec<BehaviorEntity>, PaginationInfo)> {
        let mut params = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        if let Some(pp) = per_page {
            params.push(("per_page", pp.to_string()));
        }
        if let Some(sb) = sort_by {
            params.push(("sort_by", sb.to_string()));
        }
        if let Some(f) = filter {
            params.push(("filter", f.to_string()));
        }
        if let Some(fp) = filter_prefix {
            params.push(("filter_prefix", fp.to_string()));
        }

        let query = if params.is_empty() {
            String::new()
        } else {
            format!(
                "?{}",
                params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&")
            )
        };

        let response = self.client.get_raw(&format!("/behaviors{}", query)).await?;
        let behaviors: Vec<BehaviorEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((behaviors, pagination))
    }

    /// List behaviors for a specific folder path
    ///
    /// # Arguments
    /// * `path` - Folder path
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    ///
    /// # Returns
    /// Tuple of (behaviors, pagination_info)
    pub async fn list_for_folder(
        &self,
        path: &str,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<BehaviorEntity>, PaginationInfo)> {
        let mut params = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        if let Some(pp) = per_page {
            params.push(("per_page", pp.to_string()));
        }

        let query = if params.is_empty() {
            String::new()
        } else {
            format!(
                "?{}",
                params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&")
            )
        };

        let response = self
            .client
            .get_raw(&format!("/behaviors/folders/{}{}", path, query))
            .await?;
        let behaviors: Vec<BehaviorEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((behaviors, pagination))
    }

    /// Get a specific behavior
    ///
    /// # Arguments
    /// * `id` - Behavior ID
    pub async fn get(&self, id: i64) -> Result<BehaviorEntity> {
        let response = self.client.get_raw(&format!("/behaviors/{}", id)).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new behavior
    ///
    /// # Arguments
    /// * `path` - Folder path (required)
    /// * `behavior` - Behavior type (required)
    /// * `value` - Behavior configuration value
    /// * `name` - Behavior name
    /// * `recursive` - Apply recursively
    ///
    /// # Returns
    /// The created behavior
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, BehaviorHandler};
    /// use serde_json::json;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = BehaviorHandler::new(client);
    /// let webhook_config = json!({"urls": ["https://example.com/hook"]});
    /// let behavior = handler.create(
    ///     "/uploads",
    ///     "webhook",
    ///     Some(webhook_config),
    ///     Some("Upload Webhook"),
    ///     Some(true)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        path: &str,
        behavior: &str,
        value: Option<serde_json::Value>,
        name: Option<&str>,
        recursive: Option<bool>,
    ) -> Result<BehaviorEntity> {
        let mut request_body = json!({
            "path": path,
            "behavior": behavior,
        });

        if let Some(v) = value {
            request_body["value"] = v;
        }
        if let Some(n) = name {
            request_body["name"] = json!(n);
        }
        if let Some(r) = recursive {
            request_body["recursive"] = json!(r);
        }

        let response = self.client.post_raw("/behaviors", request_body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update a behavior
    ///
    /// # Arguments
    /// * `id` - Behavior ID
    /// * `value` - New behavior configuration value
    /// * `name` - New behavior name
    /// * `disable_parent_folder_behavior` - Disable parent folder behavior
    ///
    /// # Returns
    /// The updated behavior
    pub async fn update(
        &self,
        id: i64,
        value: Option<serde_json::Value>,
        name: Option<&str>,
        disable_parent_folder_behavior: Option<bool>,
    ) -> Result<BehaviorEntity> {
        let mut request_body = json!({});

        if let Some(v) = value {
            request_body["value"] = v;
        }
        if let Some(n) = name {
            request_body["name"] = json!(n);
        }
        if let Some(d) = disable_parent_folder_behavior {
            request_body["disable_parent_folder_behavior"] = json!(d);
        }

        let response = self
            .client
            .patch_raw(&format!("/behaviors/{}", id), request_body)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a behavior
    ///
    /// # Arguments
    /// * `id` - Behavior ID
    pub async fn delete(&self, id: i64) -> Result<()> {
        self.client
            .delete_raw(&format!("/behaviors/{}", id))
            .await?;
        Ok(())
    }

    /// Test a webhook behavior
    ///
    /// # Arguments
    /// * `url` - Webhook URL to test
    /// * `method` - HTTP method (GET or POST)
    /// * `encoding` - Webhook encoding type
    /// * `headers` - Custom headers
    /// * `body` - Request body parameters
    ///
    /// # Returns
    /// Test result
    #[allow(clippy::too_many_arguments)]
    pub async fn test_webhook(
        &self,
        url: &str,
        method: Option<&str>,
        encoding: Option<&str>,
        headers: Option<serde_json::Value>,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let mut request_body = json!({
            "url": url,
        });

        if let Some(m) = method {
            request_body["method"] = json!(m);
        }
        if let Some(e) = encoding {
            request_body["encoding"] = json!(e);
        }
        if let Some(h) = headers {
            request_body["headers"] = h;
        }
        if let Some(b) = body {
            request_body["body"] = b;
        }

        let response = self
            .client
            .post_raw("/behaviors/webhook/test", request_body)
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = BehaviorHandler::new(client);
    }
}
