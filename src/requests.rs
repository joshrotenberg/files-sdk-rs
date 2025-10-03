//! Request (File Request) operations
//!
//! Requests are files that should be uploaded by a specific user or group.
//! They can be manually created/managed or automatically managed by automations.

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// A Request entity (File Request)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestEntity {
    /// Request ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Folder path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Source filename (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Destination filename
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

    /// ID of automation that created this request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_id: Option<i64>,

    /// User making the request (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_display_name: Option<String>,
}

/// Handler for request operations
pub struct RequestHandler {
    client: FilesClient,
}

impl RequestHandler {
    /// Create a new request handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List requests
    ///
    /// # Arguments
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page (max 10,000)
    /// * `path` - Filter by path
    /// * `mine` - Only show requests for current user
    ///
    /// # Returns
    /// Tuple of (requests, pagination_info)
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, RequestHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = RequestHandler::new(client);
    /// let (requests, pagination) = handler.list(None, None, None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        cursor: Option<&str>,
        per_page: Option<i64>,
        path: Option<&str>,
        mine: Option<bool>,
    ) -> Result<(Vec<RequestEntity>, PaginationInfo)> {
        let mut params = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        if let Some(pp) = per_page {
            params.push(("per_page", pp.to_string()));
        }
        if let Some(p) = path {
            params.push(("path", p.to_string()));
        }
        if let Some(m) = mine {
            params.push(("mine", m.to_string()));
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

        let response = self.client.get_raw(&format!("/requests{}", query)).await?;
        let requests: Vec<RequestEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((requests, pagination))
    }

    /// List requests for a specific folder path
    ///
    /// # Arguments
    /// * `path` - Folder path
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    ///
    /// # Returns
    /// Tuple of (requests, pagination_info)
    pub async fn list_for_folder(
        &self,
        path: &str,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<RequestEntity>, PaginationInfo)> {
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
            .get_raw(&format!("/requests/folders/{}{}", path, query))
            .await?;
        let requests: Vec<RequestEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((requests, pagination))
    }

    /// Create a new request
    ///
    /// # Arguments
    /// * `path` - Folder path where file should be uploaded (required)
    /// * `destination` - Destination filename without extension (required)
    /// * `user_ids` - Comma-separated list of user IDs to request from
    /// * `group_ids` - Comma-separated list of group IDs to request from
    ///
    /// # Returns
    /// The created request
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, RequestHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = RequestHandler::new(client);
    /// let request = handler.create(
    ///     "/uploads",
    ///     "monthly_report",
    ///     Some("123,456"),
    ///     None
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        path: &str,
        destination: &str,
        user_ids: Option<&str>,
        group_ids: Option<&str>,
    ) -> Result<RequestEntity> {
        let mut body = json!({
            "path": path,
            "destination": destination,
        });

        if let Some(uids) = user_ids {
            body["user_ids"] = json!(uids);
        }
        if let Some(gids) = group_ids {
            body["group_ids"] = json!(gids);
        }

        let response = self.client.post_raw("/requests", body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a request
    ///
    /// # Arguments
    /// * `id` - Request ID
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, RequestHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = RequestHandler::new(client);
    /// handler.delete(123).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, id: i64) -> Result<()> {
        self.client.delete_raw(&format!("/requests/{}", id)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = RequestHandler::new(client);
    }
}
