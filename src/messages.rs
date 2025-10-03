//! Message operations
//!
//! Messages are part of Files.com's project management features,
//! representing messages posted by users to projects.

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// A Message Comment entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCommentEntity {
    /// Comment ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Comment body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Reactions to this comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<serde_json::Value>>,
}

/// A Message entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Message ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Message subject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// Message body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Comments on this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<MessageCommentEntity>>,
}

/// Handler for message operations
pub struct MessageHandler {
    client: FilesClient,
}

impl MessageHandler {
    /// Create a new message handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List messages
    ///
    /// # Arguments
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    /// * `project_id` - Filter by project ID
    ///
    /// # Returns
    /// Tuple of (messages, pagination_info)
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, MessageHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = MessageHandler::new(client);
    /// let (messages, _) = handler.list(None, None, Some(1)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        cursor: Option<&str>,
        per_page: Option<i64>,
        project_id: Option<i64>,
    ) -> Result<(Vec<MessageEntity>, PaginationInfo)> {
        let mut params = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        if let Some(pp) = per_page {
            params.push(("per_page", pp.to_string()));
        }
        if let Some(pid) = project_id {
            params.push(("project_id", pid.to_string()));
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

        let response = self.client.get_raw(&format!("/messages{}", query)).await?;
        let messages: Vec<MessageEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((messages, pagination))
    }

    /// Get a specific message
    ///
    /// # Arguments
    /// * `id` - Message ID
    ///
    /// # Returns
    /// The message entity
    pub async fn get(&self, id: i64) -> Result<MessageEntity> {
        let response = self.client.get_raw(&format!("/messages/{}", id)).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new message
    ///
    /// # Arguments
    /// * `project_id` - Project ID (required)
    /// * `subject` - Message subject (required)
    /// * `body` - Message body (required)
    ///
    /// # Returns
    /// The created message
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, MessageHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = MessageHandler::new(client);
    /// let message = handler.create(
    ///     1,
    ///     "Project Update",
    ///     "Here's the latest status..."
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        project_id: i64,
        subject: &str,
        body: &str,
    ) -> Result<MessageEntity> {
        let request_body = json!({
            "project_id": project_id,
            "subject": subject,
            "body": body,
        });

        let response = self.client.post_raw("/messages", request_body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update a message
    ///
    /// # Arguments
    /// * `id` - Message ID
    /// * `subject` - New subject (optional)
    /// * `body` - New body (optional)
    ///
    /// # Returns
    /// The updated message
    pub async fn update(
        &self,
        id: i64,
        subject: Option<&str>,
        body: Option<&str>,
    ) -> Result<MessageEntity> {
        let mut request_body = json!({});

        if let Some(s) = subject {
            request_body["subject"] = json!(s);
        }
        if let Some(b) = body {
            request_body["body"] = json!(b);
        }

        let response = self
            .client
            .patch_raw(&format!("/messages/{}", id), request_body)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a message
    ///
    /// # Arguments
    /// * `id` - Message ID
    pub async fn delete(&self, id: i64) -> Result<()> {
        self.client.delete_raw(&format!("/messages/{}", id)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = MessageHandler::new(client);
    }
}
