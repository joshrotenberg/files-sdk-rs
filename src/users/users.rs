//! User management operations
//!
//! This module provides user management functionality including:
//! - List users with filtering and pagination
//! - Create new users
//! - Update user settings
//! - Delete users
//! - User utility operations (unlock, reset 2FA, etc.)

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// User entity from Files.com API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntity {
    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// User's email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// User's full name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Company name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,

    /// Notes about the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// User home directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_home: Option<String>,

    /// User root directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_root: Option<String>,

    /// Is site admin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,

    /// Is read-only site admin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_site_admin: Option<bool>,

    /// User is disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    /// User is disabled, expired, or inactive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_expired_or_inactive: Option<bool>,

    /// SSL/TLS is required for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_required: Option<String>,

    /// Time zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    /// Language preference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Allowed IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<String>,

    /// Bypass site allowed IPs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_site_allowed_ips: Option<bool>,

    /// Group IDs this user belongs to (can be string or array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,

    /// Admin group IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_group_ids: Option<Vec<i64>>,

    /// FTP permission
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftp_permission: Option<bool>,

    /// SFTP permission
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sftp_permission: Option<bool>,

    /// WebDAV permission
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dav_permission: Option<bool>,

    /// REST API permission
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restapi_permission: Option<bool>,

    /// Require 2FA
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_2fa: Option<String>,

    /// Active 2FA method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_2fa: Option<bool>,

    /// Created at timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Last login at timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_at: Option<String>,

    /// Password set at timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_set_at: Option<String>,

    /// Password validity in days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_validity_days: Option<i64>,

    /// API keys count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_keys_count: Option<i64>,

    /// Public keys count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_keys_count: Option<i64>,
}

/// Handler for user operations
#[derive(Debug, Clone)]
pub struct UserHandler {
    client: FilesClient,
}

impl UserHandler {
    /// Creates a new UserHandler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List users
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor (optional)
    /// * `per_page` - Results per page (optional, default 100)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, UserHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = UserHandler::new(client);
    /// let (users, pagination) = handler.list(None, None).await?;
    ///
    /// for user in users {
    ///     println!("User: {:?}", user.username);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        cursor: Option<String>,
        per_page: Option<i32>,
    ) -> Result<(Vec<UserEntity>, PaginationInfo)> {
        let mut path = "/users?".to_string();

        if let Some(c) = cursor {
            path.push_str(&format!("cursor={}&", c));
        }
        if let Some(pp) = per_page {
            path.push_str(&format!("per_page={}&", pp));
        }

        let response = self.client.get_raw(&path).await?;
        let users: Vec<UserEntity> = serde_json::from_value(response)?;

        // TODO: Extract pagination info from response headers
        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((users, pagination))
    }

    /// Get a specific user by ID
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    pub async fn get(&self, id: i64) -> Result<UserEntity> {
        let path = format!("/users/{}", id);
        let response = self.client.get_raw(&path).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new user
    ///
    /// # Arguments
    ///
    /// * `username` - Username (required)
    /// * `email` - Email address (optional)
    /// * `password` - Password (optional)
    /// * `name` - Full name (optional)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, UserHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = UserHandler::new(client);
    /// let user = handler.create(
    ///     "newuser",
    ///     Some("user@example.com"),
    ///     Some("password123"),
    ///     Some("New User")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        username: &str,
        email: Option<&str>,
        password: Option<&str>,
        name: Option<&str>,
    ) -> Result<UserEntity> {
        let mut body = json!({
            "username": username,
        });

        if let Some(e) = email {
            body["email"] = json!(e);
        }
        if let Some(p) = password {
            body["password"] = json!(p);
        }
        if let Some(n) = name {
            body["name"] = json!(n);
        }

        let response = self.client.post_raw("/users", body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update a user
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    /// * `email` - New email (optional)
    /// * `name` - New name (optional)
    /// * `company` - New company (optional)
    /// * `notes` - New notes (optional)
    pub async fn update(
        &self,
        id: i64,
        email: Option<&str>,
        name: Option<&str>,
        company: Option<&str>,
        notes: Option<&str>,
    ) -> Result<UserEntity> {
        let mut body = json!({});

        if let Some(e) = email {
            body["email"] = json!(e);
        }
        if let Some(n) = name {
            body["name"] = json!(n);
        }
        if let Some(c) = company {
            body["company"] = json!(c);
        }
        if let Some(nt) = notes {
            body["notes"] = json!(nt);
        }

        let path = format!("/users/{}", id);
        let response = self.client.patch_raw(&path, body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a user
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    pub async fn delete(&self, id: i64) -> Result<()> {
        let path = format!("/users/{}", id);
        self.client.delete_raw(&path).await?;
        Ok(())
    }

    /// Unlock a locked user
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    pub async fn unlock(&self, id: i64) -> Result<()> {
        let path = format!("/users/{}/unlock", id);
        self.client.post_raw(&path, json!({})).await?;
        Ok(())
    }

    /// Reset 2FA for a user
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    pub async fn reset_2fa(&self, id: i64) -> Result<()> {
        let path = format!("/users/{}/2fa/reset", id);
        self.client.post_raw(&path, json!({})).await?;
        Ok(())
    }

    /// Resend welcome email to a user
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    pub async fn resend_welcome_email(&self, id: i64) -> Result<()> {
        let path = format!("/users/{}/resend_welcome_email", id);
        self.client.post_raw(&path, json!({})).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = UserHandler::new(client);
    }
}
