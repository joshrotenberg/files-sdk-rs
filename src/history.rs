//! History operations
//!
//! History represents activity logs and audit trails. History queries must be
//! exported for processing.

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// A History Export entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryExportEntity {
    /// History Export ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Version of the history for the export
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_version: Option<String>,

    /// Start date/time of export range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,

    /// End date/time of export range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,

    /// Status (building, ready, failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Filter by action type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_action: Option<String>,

    /// Filter by interface type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_interface: Option<String>,

    /// Filter by user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_user_id: Option<String>,

    /// Filter by file ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_file_id: Option<String>,

    /// Filter by parent folder ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parent_id: Option<String>,

    /// Filter by path pattern
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_path: Option<String>,

    /// Filter by folder pattern
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_folder: Option<String>,

    /// Filter by source pattern (for moves)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_src: Option<String>,

    /// Filter by destination pattern (for moves)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_destination: Option<String>,

    /// Filter by IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_ip: Option<String>,

    /// Filter by username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_username: Option<String>,

    /// Filter by failure type (for login failures)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_failure_type: Option<String>,

    /// Filter by target object ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_target_id: Option<String>,

    /// Filter by target object name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_target_name: Option<String>,

    /// Filter by target permission level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_target_permission: Option<String>,

    /// Filter by target user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_target_user_id: Option<String>,

    /// Filter by target username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_target_username: Option<String>,

    /// Filter by target platform
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_target_platform: Option<String>,

    /// Filter by target permission set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_target_permission_set: Option<String>,

    /// Results download URL (when ready)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_url: Option<String>,
}

/// A History Export Result entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryExportResultEntity {
    /// ID of the export this result belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_export_id: Option<i64>,

    /// Result data (varies by export type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

/// Handler for history operations
pub struct HistoryHandler {
    client: FilesClient,
}

impl HistoryHandler {
    /// Create a new history handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List history for a specific file path
    ///
    /// # Arguments
    /// * `path` - File path
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    ///
    /// # Returns
    /// Tuple of (history_items, pagination_info)
    pub async fn list_for_file(
        &self,
        path: &str,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<serde_json::Value>, PaginationInfo)> {
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
            .get_raw(&format!("/history/files/{}{}", path, query))
            .await?;
        let history: Vec<serde_json::Value> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((history, pagination))
    }

    /// List history for a specific folder path
    ///
    /// # Arguments
    /// * `path` - Folder path
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    pub async fn list_for_folder(
        &self,
        path: &str,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<serde_json::Value>, PaginationInfo)> {
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
            .get_raw(&format!("/history/folders/{}{}", path, query))
            .await?;
        let history: Vec<serde_json::Value> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((history, pagination))
    }

    /// List history for a specific user
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    pub async fn list_for_user(
        &self,
        user_id: i64,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<serde_json::Value>, PaginationInfo)> {
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
            .get_raw(&format!("/history/users/{}{}", user_id, query))
            .await?;
        let history: Vec<serde_json::Value> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((history, pagination))
    }

    /// List login history
    ///
    /// # Arguments
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    pub async fn list_logins(
        &self,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<serde_json::Value>, PaginationInfo)> {
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
            .get_raw(&format!("/history/login{}", query))
            .await?;
        let history: Vec<serde_json::Value> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((history, pagination))
    }

    /// Create a history export
    ///
    /// # Arguments
    /// * `start_at` - Start date/time
    /// * `end_at` - End date/time
    /// * `query_action` - Filter by action
    /// * `query_user_id` - Filter by user ID
    /// * `query_folder` - Filter by folder
    ///
    /// # Returns
    /// The created history export
    #[allow(clippy::too_many_arguments)]
    pub async fn create_export(
        &self,
        start_at: Option<&str>,
        end_at: Option<&str>,
        query_action: Option<&str>,
        query_user_id: Option<&str>,
        query_folder: Option<&str>,
    ) -> Result<HistoryExportEntity> {
        let mut request_body = json!({});

        if let Some(start) = start_at {
            request_body["start_at"] = json!(start);
        }
        if let Some(end) = end_at {
            request_body["end_at"] = json!(end);
        }
        if let Some(action) = query_action {
            request_body["query_action"] = json!(action);
        }
        if let Some(user) = query_user_id {
            request_body["query_user_id"] = json!(user);
        }
        if let Some(folder) = query_folder {
            request_body["query_folder"] = json!(folder);
        }

        let response = self
            .client
            .post_raw("/history_exports", request_body)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Get a history export by ID
    ///
    /// # Arguments
    /// * `id` - History export ID
    pub async fn get_export(&self, id: i64) -> Result<HistoryExportEntity> {
        let response = self
            .client
            .get_raw(&format!("/history_exports/{}", id))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Get history export results
    ///
    /// # Arguments
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    /// * `history_export_id` - Filter by export ID
    pub async fn get_export_results(
        &self,
        cursor: Option<&str>,
        per_page: Option<i64>,
        history_export_id: Option<i64>,
    ) -> Result<(Vec<HistoryExportResultEntity>, PaginationInfo)> {
        let mut params = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        if let Some(pp) = per_page {
            params.push(("per_page", pp.to_string()));
        }
        if let Some(export_id) = history_export_id {
            params.push(("history_export_id", export_id.to_string()));
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
            .get_raw(&format!("/history_export_results{}", query))
            .await?;
        let results: Vec<HistoryExportResultEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((results, pagination))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = HistoryHandler::new(client);
    }
}
