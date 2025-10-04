//! Session management operations
//!
//! This module provides session management functionality including:
//! - Create sessions (login with username/password)
//! - Delete sessions (logout)
//!
//! Note: Most SDK users will use API key authentication instead of sessions.
//! Sessions are primarily for user-facing applications that need login flows.

use crate::{FilesClient, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Session entity from Files.com API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEntity {
    /// Session ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Language preference for this session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Is this a read-only session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// Allow insecure SFTP ciphers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sftp_insecure_ciphers: Option<bool>,
}

/// Handler for session operations
#[derive(Debug, Clone)]
pub struct SessionHandler {
    client: FilesClient,
}

impl SessionHandler {
    /// Creates a new SessionHandler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// Create a new session (login)
    ///
    /// # Arguments
    ///
    /// * `username` - Username to sign in as
    /// * `password` - Password for sign in
    /// * `otp` - One-time password for 2FA (optional)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, SessionHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = SessionHandler::new(client);
    /// let session = handler.create("username", "password", None).await?;
    /// println!("Session ID: {:?}", session.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        username: &str,
        password: &str,
        otp: Option<&str>,
    ) -> Result<SessionEntity> {
        let mut body = json!({
            "username": username,
            "password": password,
        });

        if let Some(o) = otp {
            body["otp"] = json!(o);
        }

        let response = self.client.post_raw("/sessions", body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a session (logout)
    ///
    /// Deletes the current session, effectively logging out the user.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, SessionHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = SessionHandler::new(client);
    /// handler.delete().await?;
    /// println!("Logged out successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self) -> Result<()> {
        self.client.delete_raw("/sessions").await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = SessionHandler::new(client);
    }
}
