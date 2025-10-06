//! User management operations
//!
//! Provides comprehensive user account management for Files.com sites, including
//! creation, modification, and administrative operations.
//!
//! # Features
//!
//! - Create and manage user accounts
//! - Configure permissions and access controls
//! - Set user quotas and restrictions
//! - Administrative operations (unlock, reset 2FA)
//! - Group membership management
//! - Protocol-specific permissions (FTP, SFTP, WebDAV, REST API)
//!
//! # Example
//!
//! ```no_run
//! use files_sdk::{FilesClient, UserHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = FilesClient::builder()
//!     .api_key("your-api-key")
//!     .build()?;
//!
//! let handler = UserHandler::new(client);
//!
//! // Create a new user with SFTP access
//! let user = handler.create(
//!     "newuser",
//!     Some("user@company.com"),
//!     Some("secure-password"),
//!     Some("New User")
//! ).await?;
//!
//! println!("Created user ID: {}", user.id.unwrap());
//!
//! // List all users
//! let (users, _) = handler.list(None, Some(100)).await?;
//! for user in users {
//!     println!("User: {} ({})",
//!         user.username.unwrap_or_default(),
//!         user.email.unwrap_or_default());
//! }
//! # Ok(())
//! # }
//! ```

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

    /// List all users on the site
    ///
    /// Returns a paginated list of user accounts with their details and permissions.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor from previous response
    /// * `per_page` - Number of results per page (default 100, max 10,000)
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - Vector of `UserEntity` objects
    /// - `PaginationInfo` with cursors for next/previous pages
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, UserHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = UserHandler::new(client);
    ///
    /// // List first page of users
    /// let (users, pagination) = handler.list(None, Some(50)).await?;
    ///
    /// for user in users {
    ///     println!("{}: {} - Admin: {}",
    ///         user.username.unwrap_or_default(),
    ///         user.email.unwrap_or_default(),
    ///         user.site_admin.unwrap_or(false));
    /// }
    ///
    /// // Get next page if available
    /// if let Some(next) = pagination.cursor_next {
    ///     let (more_users, _) = handler.list(Some(next), Some(50)).await?;
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

    /// Get details of a specific user by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The unique user ID
    ///
    /// # Returns
    ///
    /// A `UserEntity` with complete user information
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, UserHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = UserHandler::new(client);
    ///
    /// let user = handler.get(12345).await?;
    /// println!("User: {}", user.username.unwrap_or_default());
    /// println!("Last login: {}", user.last_login_at.unwrap_or_default());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, id: i64) -> Result<UserEntity> {
        let path = format!("/users/{}", id);
        let response = self.client.get_raw(&path).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new user account
    ///
    /// Creates a new user with the specified credentials. The user will receive
    /// a welcome email if email is provided.
    ///
    /// # Arguments
    ///
    /// * `username` - Unique username for login (required)
    /// * `email` - User's email address
    /// * `password` - Initial password (if not provided, user must set on first login)
    /// * `name` - User's full name
    ///
    /// # Returns
    ///
    /// The newly created `UserEntity`
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, UserHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = UserHandler::new(client);
    ///
    /// // Create user with all details
    /// let user = handler.create(
    ///     "jdoe",
    ///     Some("jdoe@company.com"),
    ///     Some("SecureP@ssw0rd"),
    ///     Some("John Doe")
    /// ).await?;
    ///
    /// println!("Created user: {} (ID: {})",
    ///     user.username.unwrap(),
    ///     user.id.unwrap());
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

    /// Update user information
    ///
    /// Updates user profile information. Only provided fields are updated;
    /// omitted fields remain unchanged.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to update
    /// * `email` - New email address
    /// * `name` - New full name
    /// * `company` - New company name
    /// * `notes` - Administrative notes about the user
    ///
    /// # Returns
    ///
    /// The updated `UserEntity`
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, UserHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = UserHandler::new(client);
    ///
    /// // Update user's company and notes
    /// let user = handler.update(
    ///     12345,
    ///     None,
    ///     None,
    ///     Some("New Company Inc."),
    ///     Some("Transferred from old company")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Delete a user account permanently
    ///
    /// Removes the user account and all associated data. This operation cannot
    /// be undone.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, UserHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = UserHandler::new(client);
    ///
    /// handler.delete(12345).await?;
    /// println!("User deleted successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, id: i64) -> Result<()> {
        let path = format!("/users/{}", id);
        self.client.delete_raw(&path).await?;
        Ok(())
    }

    /// Unlock a locked user account
    ///
    /// Removes the lock from a user account that has been locked due to too many
    /// failed login attempts or administrative action.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID to unlock
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, UserHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = UserHandler::new(client);
    ///
    /// // Unlock user after failed login attempts
    /// handler.unlock(12345).await?;
    /// println!("User unlocked successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unlock(&self, id: i64) -> Result<()> {
        let path = format!("/users/{}/unlock", id);
        self.client.post_raw(&path, json!({})).await?;
        Ok(())
    }

    /// Reset two-factor authentication for a user
    ///
    /// Disables 2FA for the user, requiring them to set it up again on next login.
    /// Use this when a user has lost access to their 2FA device.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, UserHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = UserHandler::new(client);
    ///
    /// // Reset 2FA for user who lost their device
    /// handler.reset_2fa(12345).await?;
    /// println!("2FA reset - user must configure on next login");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reset_2fa(&self, id: i64) -> Result<()> {
        let path = format!("/users/{}/2fa/reset", id);
        self.client.post_raw(&path, json!({})).await?;
        Ok(())
    }

    /// Resend welcome email to a user
    ///
    /// Sends a new welcome email to the user with login instructions and
    /// password setup link if needed.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, UserHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = UserHandler::new(client);
    ///
    /// // Resend welcome email to new user
    /// handler.resend_welcome_email(12345).await?;
    /// println!("Welcome email sent");
    /// # Ok(())
    /// # }
    /// ```
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
