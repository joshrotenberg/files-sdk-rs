//! Group management operations
//!
//! This module provides group management functionality including:
//! - List groups
//! - Create new groups
//! - Update group settings
//! - Delete groups
//! - Manage group memberships

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Group entity from Files.com API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEntity {
    /// Group ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Group name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Notes about the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Admin user IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_ids: Option<Vec<i64>>,

    /// User IDs in this group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<i64>>,

    /// Usernames in this group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usernames: Option<Vec<String>>,

    /// Allowed IP addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<String>,

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

    /// Site ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<i64>,
}

/// Handler for group operations
#[derive(Debug, Clone)]
pub struct GroupHandler {
    client: FilesClient,
}

impl GroupHandler {
    /// Creates a new GroupHandler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List groups
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor (optional)
    /// * `per_page` - Results per page (optional)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, GroupHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = GroupHandler::new(client);
    /// let (groups, pagination) = handler.list(None, Some(10)).await?;
    ///
    /// for group in groups {
    ///     println!("Group: {:?}", group.name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        cursor: Option<String>,
        per_page: Option<i32>,
    ) -> Result<(Vec<GroupEntity>, PaginationInfo)> {
        let mut path = "/groups?".to_string();

        if let Some(c) = cursor {
            path.push_str(&format!("cursor={}&", c));
        }
        if let Some(pp) = per_page {
            path.push_str(&format!("per_page={}&", pp));
        }

        let response = self.client.get_raw(&path).await?;
        let groups: Vec<GroupEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((groups, pagination))
    }

    /// Get a specific group by ID
    ///
    /// # Arguments
    ///
    /// * `id` - Group ID
    pub async fn get(&self, id: i64) -> Result<GroupEntity> {
        let path = format!("/groups/{}", id);
        let response = self.client.get_raw(&path).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new group
    ///
    /// # Arguments
    ///
    /// * `name` - Group name (required)
    /// * `notes` - Notes about the group (optional)
    /// * `user_ids` - User IDs to add to group (optional)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, GroupHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = GroupHandler::new(client);
    /// let group = handler.create(
    ///     "Developers",
    ///     Some("Development team members"),
    ///     None
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        name: &str,
        notes: Option<&str>,
        user_ids: Option<Vec<i64>>,
    ) -> Result<GroupEntity> {
        let mut body = json!({
            "name": name,
        });

        if let Some(n) = notes {
            body["notes"] = json!(n);
        }
        if let Some(uids) = user_ids {
            body["user_ids"] = json!(uids);
        }

        let response = self.client.post_raw("/groups", body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update a group
    ///
    /// # Arguments
    ///
    /// * `id` - Group ID
    /// * `name` - New name (optional)
    /// * `notes` - New notes (optional)
    pub async fn update(
        &self,
        id: i64,
        name: Option<&str>,
        notes: Option<&str>,
    ) -> Result<GroupEntity> {
        let mut body = json!({});

        if let Some(n) = name {
            body["name"] = json!(n);
        }
        if let Some(nt) = notes {
            body["notes"] = json!(nt);
        }

        let path = format!("/groups/{}", id);
        let response = self.client.patch_raw(&path, body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a group
    ///
    /// # Arguments
    ///
    /// * `id` - Group ID
    pub async fn delete(&self, id: i64) -> Result<()> {
        let path = format!("/groups/{}", id);
        self.client.delete_raw(&path).await?;
        Ok(())
    }

    /// Add a user to a group
    ///
    /// # Arguments
    ///
    /// * `group_id` - Group ID
    /// * `user_id` - User ID to add
    pub async fn add_user(&self, group_id: i64, user_id: i64) -> Result<()> {
        let path = format!("/groups/{}/users", group_id);
        let body = json!({
            "user_id": user_id,
        });
        self.client.post_raw(&path, body).await?;
        Ok(())
    }

    /// Remove a user from a group
    ///
    /// # Arguments
    ///
    /// * `group_id` - Group ID
    /// * `user_id` - User ID to remove
    pub async fn remove_user(&self, group_id: i64, user_id: i64) -> Result<()> {
        let path = format!("/groups/{}/memberships/{}", group_id, user_id);
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
        let _handler = GroupHandler::new(client);
    }
}
