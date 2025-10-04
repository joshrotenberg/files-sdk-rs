//! Notification operations
//!
//! Notifications send emails when specific actions occur in folders.
//! Emails are sent in batches at configured intervals (5 min, 15 min, hourly, daily).

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Notification send interval enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SendInterval {
    /// Every 5 minutes
    FiveMinutes,
    /// Every 15 minutes
    FifteenMinutes,
    /// Hourly
    Hourly,
    /// Daily
    Daily,
}

/// Unsubscribe reason enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsubscribedReason {
    /// Not unsubscribed
    None,
    /// User clicked unsubscribe link
    UnsubscribeLinkClicked,
    /// Mail bounced
    MailBounced,
    /// Mail marked as spam
    MailMarkedAsSpam,
}

/// A Notification entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEntity {
    /// Notification ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Folder path to notify on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Group ID to receive notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,

    /// Group name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// Only notify on actions by these groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggering_group_ids: Option<Vec<i64>>,

    /// Only notify on actions by these users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggering_user_ids: Option<Vec<i64>>,

    /// Notify on share recipient actions?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by_share_recipients: Option<bool>,

    /// Send notifications about user's own activity?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_user_actions: Option<bool>,

    /// Trigger on file copy?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_copy: Option<bool>,

    /// Trigger on file delete?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_delete: Option<bool>,

    /// Trigger on file download?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_download: Option<bool>,

    /// Trigger on file move?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_move: Option<bool>,

    /// Trigger on file upload/update?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_upload: Option<bool>,

    /// Apply recursively to subfolders?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,

    /// Email send interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_interval: Option<String>,

    /// Custom message in notification emails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Filenames to trigger on (with wildcards)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggering_filenames: Option<Vec<String>>,

    /// Is user unsubscribed?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<bool>,

    /// Unsubscribe reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed_reason: Option<String>,

    /// User ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// Username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Email suppressed due to bounce/spam?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_email: Option<bool>,
}

/// Handler for notification operations
pub struct NotificationHandler {
    client: FilesClient,
}

impl NotificationHandler {
    /// Create a new notification handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List notifications
    ///
    /// # Arguments
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    /// * `path` - Filter by path
    /// * `group_id` - Filter by group ID
    ///
    /// # Returns
    /// Tuple of (notifications, pagination_info)
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, NotificationHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = NotificationHandler::new(client);
    /// let (notifications, _) = handler.list(None, None, None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        cursor: Option<&str>,
        per_page: Option<i64>,
        path: Option<&str>,
        group_id: Option<i64>,
    ) -> Result<(Vec<NotificationEntity>, PaginationInfo)> {
        let mut params = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        if let Some(pp) = per_page {
            params.push(("per_page", pp.to_string()));
        }
        if let Some(p) = path {
            params.push(("path", p.to_string()));
        }
        if let Some(gid) = group_id {
            params.push(("group_id", gid.to_string()));
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
            .get_raw(&format!("/notifications{}", query))
            .await?;
        let notifications: Vec<NotificationEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((notifications, pagination))
    }

    /// Get a specific notification
    ///
    /// # Arguments
    /// * `id` - Notification ID
    ///
    /// # Returns
    /// The notification entity
    pub async fn get(&self, id: i64) -> Result<NotificationEntity> {
        let response = self
            .client
            .get_raw(&format!("/notifications/{}", id))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new notification
    ///
    /// # Arguments
    /// * `path` - Folder path to monitor
    /// * `group_id` - Group to notify
    /// * `notify_on_upload` - Trigger on uploads
    /// * `notify_on_download` - Trigger on downloads
    /// * `notify_on_delete` - Trigger on deletes
    /// * `send_interval` - Email frequency
    /// * `recursive` - Apply to subfolders
    ///
    /// # Returns
    /// The created notification
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        path: Option<&str>,
        group_id: Option<i64>,
        notify_on_upload: Option<bool>,
        notify_on_download: Option<bool>,
        notify_on_delete: Option<bool>,
        send_interval: Option<&str>,
        recursive: Option<bool>,
        message: Option<&str>,
    ) -> Result<NotificationEntity> {
        let mut body = json!({});

        if let Some(p) = path {
            body["path"] = json!(p);
        }
        if let Some(gid) = group_id {
            body["group_id"] = json!(gid);
        }
        if let Some(u) = notify_on_upload {
            body["notify_on_upload"] = json!(u);
        }
        if let Some(d) = notify_on_download {
            body["notify_on_download"] = json!(d);
        }
        if let Some(del) = notify_on_delete {
            body["notify_on_delete"] = json!(del);
        }
        if let Some(si) = send_interval {
            body["send_interval"] = json!(si);
        }
        if let Some(r) = recursive {
            body["recursive"] = json!(r);
        }
        if let Some(m) = message {
            body["message"] = json!(m);
        }

        let response = self.client.post_raw("/notifications", body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update a notification
    ///
    /// # Arguments
    /// * `id` - Notification ID
    /// * `notify_on_upload` - Trigger on uploads
    /// * `notify_on_download` - Trigger on downloads
    /// * `notify_on_delete` - Trigger on deletes
    /// * `send_interval` - Email frequency
    ///
    /// # Returns
    /// The updated notification
    pub async fn update(
        &self,
        id: i64,
        notify_on_upload: Option<bool>,
        notify_on_download: Option<bool>,
        notify_on_delete: Option<bool>,
        send_interval: Option<&str>,
    ) -> Result<NotificationEntity> {
        let mut body = json!({});

        if let Some(u) = notify_on_upload {
            body["notify_on_upload"] = json!(u);
        }
        if let Some(d) = notify_on_download {
            body["notify_on_download"] = json!(d);
        }
        if let Some(del) = notify_on_delete {
            body["notify_on_delete"] = json!(del);
        }
        if let Some(si) = send_interval {
            body["send_interval"] = json!(si);
        }

        let response = self
            .client
            .patch_raw(&format!("/notifications/{}", id), body)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a notification
    ///
    /// # Arguments
    /// * `id` - Notification ID
    pub async fn delete(&self, id: i64) -> Result<()> {
        self.client
            .delete_raw(&format!("/notifications/{}", id))
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
        let _handler = NotificationHandler::new(client);
    }
}
