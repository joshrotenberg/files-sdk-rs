//! Inbox Upload operations
//!
//! InboxUpload is a log record about upload operations that happened in an Inbox.

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};

/// An Inbox Registration entity (embedded in InboxUpload)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxRegistrationEntity {
    /// Registration code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Registrant name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Registrant company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,

    /// Registrant email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Registrant IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Clickwrap agreement body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clickwrap_body: Option<String>,

    /// Form field set ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_field_set_id: Option<i64>,

    /// Form field data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_field_data: Option<serde_json::Value>,

    /// Inbox ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<i64>,

    /// Inbox recipient ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_recipient_id: Option<i64>,

    /// Inbox title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_title: Option<String>,

    /// Registration creation time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// An Inbox Upload entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxUploadEntity {
    /// Inbox registration details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_registration: Option<InboxRegistrationEntity>,

    /// Upload path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Upload date/time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Handler for inbox upload operations
pub struct InboxUploadHandler {
    client: FilesClient,
}

impl InboxUploadHandler {
    /// Create a new inbox upload handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List inbox uploads
    ///
    /// # Arguments
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    /// * `sort_by` - Sort field and direction (e.g., `{"created_at": "desc"}`)
    /// * `filter` - Filter criteria (e.g., `{"folder_behavior_id": 123}`)
    /// * `inbox_registration_id` - Filter by inbox registration ID
    /// * `inbox_id` - Filter by inbox ID
    ///
    /// # Returns
    /// Tuple of (inbox_uploads, pagination_info)
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, InboxUploadHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = InboxUploadHandler::new(client);
    /// let (uploads, _) = handler.list(None, None, None, None, None, None).await?;
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
        inbox_registration_id: Option<i64>,
        inbox_id: Option<i64>,
    ) -> Result<(Vec<InboxUploadEntity>, PaginationInfo)> {
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
        if let Some(irid) = inbox_registration_id {
            params.push(("inbox_registration_id", irid.to_string()));
        }
        if let Some(iid) = inbox_id {
            params.push(("inbox_id", iid.to_string()));
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
            .get_raw(&format!("/inbox_uploads{}", query))
            .await?;
        let uploads: Vec<InboxUploadEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((uploads, pagination))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = InboxUploadHandler::new(client);
    }
}
