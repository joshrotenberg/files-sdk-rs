//! Bundle (Share Link) operations
//!
//! Bundles are the API/SDK term for Share Links in the Files.com web interface.
//! They allow you to share files and folders with external users via a public URL.

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Bundle permissions enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundlePermission {
    /// Read-only access
    Read,
    /// Write-only access (upload)
    Write,
    /// Read and write access
    ReadWrite,
    /// Full access
    Full,
    /// No access
    None,
    /// Preview only (no download)
    PreviewOnly,
}

/// A Bundle entity (Share Link)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleEntity {
    /// Bundle ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Bundle code - forms the end part of the public URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Public URL of share link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Public description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Bundle internal note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    /// Is password protected?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_protected: Option<bool>,

    /// Permissions that apply to folders in this share link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,

    /// Preview only mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_only: Option<bool>,

    /// Require registration to access?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_registration: Option<bool>,

    /// Require explicit share recipient?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_share_recipient: Option<bool>,

    /// Require logout after each access?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_logout: Option<bool>,

    /// Legal clickwrap text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clickwrap_body: Option<String>,

    /// ID of clickwrap to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clickwrap_id: Option<i64>,

    /// Skip name in registration?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_name: Option<bool>,

    /// Skip email in registration?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_email: Option<bool>,

    /// Skip company in registration?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_company: Option<bool>,

    /// Bundle expiration date/time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    /// Date when share becomes accessible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_access_on_date: Option<String>,

    /// Bundle created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Don't create subfolders for submissions?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_separate_submissions_by_folder: Option<bool>,

    /// Maximum number of uses
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<i64>,

    /// Template for submission subfolder paths
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_template: Option<String>,

    /// Timezone for path template timestamps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_template_time_zone: Option<String>,

    /// Send receipt to uploader?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_receipt_to_uploader: Option<bool>,

    /// Snapshot ID containing bundle contents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<i64>,

    /// Bundle creator user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// Bundle creator username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Associated inbox ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<i64>,

    /// Has associated inbox?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_inbox: Option<bool>,

    /// Prevent folder uploads?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_allow_folders_in_uploads: Option<bool>,

    /// Paths included in bundle (not provided when listing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,

    /// Page link and button color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_left: Option<String>,

    /// Top bar link color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_link: Option<String>,

    /// Page link and button color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_text: Option<String>,

    /// Top bar background color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_top: Option<String>,

    /// Top bar text color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_top_text: Option<String>,
}

/// Handler for bundle operations
pub struct BundleHandler {
    client: FilesClient,
}

impl BundleHandler {
    /// Create a new bundle handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List bundles
    ///
    /// # Arguments
    /// * `user_id` - Filter by user ID (0 for current user)
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    ///
    /// # Returns
    /// Tuple of (bundles, pagination_info)
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, BundleHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = BundleHandler::new(client);
    /// let (bundles, pagination) = handler.list(None, None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        user_id: Option<i64>,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<BundleEntity>, PaginationInfo)> {
        let mut params = vec![];
        if let Some(uid) = user_id {
            params.push(("user_id", uid.to_string()));
        }
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

        let response = self.client.get_raw(&format!("/bundles{}", query)).await?;
        let bundles: Vec<BundleEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((bundles, pagination))
    }

    /// Get a specific bundle
    ///
    /// # Arguments
    /// * `id` - Bundle ID
    ///
    /// # Returns
    /// The bundle entity
    pub async fn get(&self, id: i64) -> Result<BundleEntity> {
        let response = self.client.get_raw(&format!("/bundles/{}", id)).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new bundle
    ///
    /// # Arguments
    /// * `paths` - List of paths to include in bundle (required)
    /// * `password` - Password protection
    /// * `expires_at` - Expiration date/time
    /// * `max_uses` - Maximum number of accesses
    /// * `description` - Public description
    /// * `note` - Internal note
    /// * `code` - Custom bundle code for URL
    /// * `require_registration` - Require user registration
    /// * `permissions` - Permission level (read, write, read_write, full, preview_only)
    ///
    /// # Returns
    /// The created bundle
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        paths: Vec<String>,
        password: Option<&str>,
        expires_at: Option<&str>,
        max_uses: Option<i64>,
        description: Option<&str>,
        note: Option<&str>,
        code: Option<&str>,
        require_registration: Option<bool>,
        permissions: Option<&str>,
    ) -> Result<BundleEntity> {
        let mut body = json!({
            "paths": paths,
        });

        if let Some(p) = password {
            body["password"] = json!(p);
        }
        if let Some(e) = expires_at {
            body["expires_at"] = json!(e);
        }
        if let Some(m) = max_uses {
            body["max_uses"] = json!(m);
        }
        if let Some(d) = description {
            body["description"] = json!(d);
        }
        if let Some(n) = note {
            body["note"] = json!(n);
        }
        if let Some(c) = code {
            body["code"] = json!(c);
        }
        if let Some(r) = require_registration {
            body["require_registration"] = json!(r);
        }
        if let Some(perm) = permissions {
            body["permissions"] = json!(perm);
        }

        let response = self.client.post_raw("/bundles", body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update a bundle
    ///
    /// # Arguments
    /// * `id` - Bundle ID
    /// * `password` - Password protection
    /// * `expires_at` - Expiration date/time
    /// * `max_uses` - Maximum number of accesses
    /// * `description` - Public description
    /// * `note` - Internal note
    ///
    /// # Returns
    /// The updated bundle
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        id: i64,
        password: Option<&str>,
        expires_at: Option<&str>,
        max_uses: Option<i64>,
        description: Option<&str>,
        note: Option<&str>,
    ) -> Result<BundleEntity> {
        let mut body = json!({});

        if let Some(p) = password {
            body["password"] = json!(p);
        }
        if let Some(e) = expires_at {
            body["expires_at"] = json!(e);
        }
        if let Some(m) = max_uses {
            body["max_uses"] = json!(m);
        }
        if let Some(d) = description {
            body["description"] = json!(d);
        }
        if let Some(n) = note {
            body["note"] = json!(n);
        }

        let response = self
            .client
            .patch_raw(&format!("/bundles/{}", id), body)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a bundle
    ///
    /// # Arguments
    /// * `id` - Bundle ID
    pub async fn delete(&self, id: i64) -> Result<()> {
        self.client.delete_raw(&format!("/bundles/{}", id)).await?;
        Ok(())
    }

    /// Share a bundle via email
    ///
    /// # Arguments
    /// * `id` - Bundle ID
    /// * `to` - Email recipients (comma-separated or array)
    /// * `note` - Optional note to include in email
    ///
    /// # Returns
    /// Success confirmation
    pub async fn share(&self, id: i64, to: Vec<String>, note: Option<&str>) -> Result<()> {
        let mut body = json!({
            "to": to,
        });

        if let Some(n) = note {
            body["note"] = json!(n);
        }

        self.client
            .post_raw(&format!("/bundles/{}/share", id), body)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = BundleHandler::new(client);
    }
}
