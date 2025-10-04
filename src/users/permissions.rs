//! Permission management for Files.com
//!
//! Permissions grant access to specific paths for users or groups.
//! They can be recursive (apply to subfolders) or non-recursive.

use crate::{FilesClient, Result};
use serde::{Deserialize, Serialize};

/// Permission types available in Files.com
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PermissionType {
    /// List, preview, read, write, move, delete, rename, manage permissions
    Admin,
    /// Share files via bundles (share links)
    Bundle,
    /// Read, write, move, delete, rename files
    Full,
    /// View history and create email notifications
    History,
    /// List files and folders only
    List,
    /// List, preview, and download files
    Readonly,
    /// Readonly site admin on child sites
    #[serde(rename = "readonly_site_admin")]
    ReadonlySiteAdmin,
    /// Site admin on child sites
    #[serde(rename = "site_admin")]
    SiteAdmin,
    /// Upload files and create folders
    Writeonly,
}

/// A permission entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionEntity {
    /// Permission ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Folder path this permission applies to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// User ID (if permission is for a user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// Username (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Group ID (if permission is for a group)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,

    /// Group name (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// Permission type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,

    /// Apply to subfolders recursively
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,

    /// Site ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<i64>,
}

/// Handler for permission operations
pub struct PermissionHandler {
    client: FilesClient,
}

impl PermissionHandler {
    /// Create a new permission handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List permissions
    ///
    /// # Arguments
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Number of records per page (max 10000)
    ///
    /// # Returns
    /// A tuple of (permissions, pagination_info)
    pub async fn list(
        &self,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<PermissionEntity>, crate::types::PaginationInfo)> {
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
            .get_raw(&format!("/permissions{}", query))
            .await?;
        let permissions: Vec<PermissionEntity> = serde_json::from_value(response)?;

        // Get pagination info from response headers if available
        let pagination = crate::types::PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((permissions, pagination))
    }

    /// Create a permission
    ///
    /// # Arguments
    /// * `path` - Folder path (required)
    /// * `permission` - Permission type (admin, full, readonly, writeonly, list, history)
    /// * `user_id` - User ID (provide user_id or username)
    /// * `username` - Username (provide user_id or username)
    /// * `group_id` - Group ID (provide group_id or group_name)
    /// * `group_name` - Group name (provide group_id or group_name)
    /// * `recursive` - Apply to subfolders recursively
    ///
    /// # Returns
    /// The created permission
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        path: &str,
        permission: Option<&str>,
        user_id: Option<i64>,
        username: Option<&str>,
        group_id: Option<i64>,
        group_name: Option<&str>,
        recursive: Option<bool>,
    ) -> Result<PermissionEntity> {
        let mut params = vec![("path", path.to_string())];

        if let Some(p) = permission {
            params.push(("permission", p.to_string()));
        }
        if let Some(uid) = user_id {
            params.push(("user_id", uid.to_string()));
        }
        if let Some(u) = username {
            params.push(("username", u.to_string()));
        }
        if let Some(gid) = group_id {
            params.push(("group_id", gid.to_string()));
        }
        if let Some(gn) = group_name {
            params.push(("group_name", gn.to_string()));
        }
        if let Some(r) = recursive {
            params.push(("recursive", r.to_string()));
        }

        let response = self.client.post_form("/permissions", &params).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a permission
    ///
    /// # Arguments
    /// * `id` - Permission ID
    pub async fn delete(&self, id: i64) -> Result<()> {
        self.client
            .delete_raw(&format!("/permissions/{}", id))
            .await?;
        Ok(())
    }

    /// List permissions for a specific user
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Number of records per page
    pub async fn list_for_user(
        &self,
        user_id: i64,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<PermissionEntity>, crate::types::PaginationInfo)> {
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
            .get_raw(&format!("/users/{}/permissions{}", user_id, query))
            .await?;
        let permissions: Vec<PermissionEntity> = serde_json::from_value(response)?;

        let pagination = crate::types::PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((permissions, pagination))
    }

    /// List permissions for a specific group
    ///
    /// # Arguments
    /// * `group_id` - Group ID
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Number of records per page
    pub async fn list_for_group(
        &self,
        group_id: i64,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<PermissionEntity>, crate::types::PaginationInfo)> {
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
            .get_raw(&format!("/groups/{}/permissions{}", group_id, query))
            .await?;
        let permissions: Vec<PermissionEntity> = serde_json::from_value(response)?;

        let pagination = crate::types::PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((permissions, pagination))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = PermissionHandler::new(client);
    }
}
