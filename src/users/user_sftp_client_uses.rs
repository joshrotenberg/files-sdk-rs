//! User SFTP Client Uses Handler
//!
//! Track SFTP client usage by users.

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};

/// Represents a user's SFTP client usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSftpClientUseEntity {
    /// UserSftpClientUse ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// The SFTP client used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sftp_client: Option<String>,

    /// The earliest recorded use of this SFTP client (for this user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// The most recent use of this SFTP client (for this user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,

    /// ID of the user who performed this access
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

/// Handler for User SFTP Client Use operations
pub struct UserSftpClientUseHandler {
    client: FilesClient,
}

impl UserSftpClientUseHandler {
    /// Creates a new UserSftpClientUseHandler
    ///
    /// # Example
    /// ```no_run
    /// # use files_sdk::{FilesClient, UserSftpClientUseHandler};
    /// let client = FilesClient::builder().api_key("key").build().unwrap();
    /// let handler = UserSftpClientUseHandler::new(client);
    /// ```
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List User SFTP Client Uses
    ///
    /// # Arguments
    /// * `user_id` - Optional user ID to filter by
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Number of records per page
    ///
    /// # Example
    /// ```no_run
    /// # use files_sdk::{FilesClient, UserSftpClientUseHandler};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = UserSftpClientUseHandler::new(client);
    ///
    /// let (uses, pagination) = handler.list(None, None, Some(100)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        user_id: Option<i64>,
        cursor: Option<String>,
        per_page: Option<i32>,
    ) -> Result<(Vec<UserSftpClientUseEntity>, PaginationInfo)> {
        let mut endpoint = "/user_sftp_client_uses".to_string();
        let mut query_params = Vec::new();

        if let Some(user_id) = user_id {
            query_params.push(format!("user_id={}", user_id));
        }

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

        let response = self.client.get_raw(&endpoint).await?;
        let items: Vec<UserSftpClientUseEntity> = serde_json::from_value(response)?;

        // TODO: Extract pagination from response headers
        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((items, pagination))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder()
            .api_key("test-key")
            .build()
            .expect("Client build failed");
        let _handler = UserSftpClientUseHandler::new(client);
    }
}
