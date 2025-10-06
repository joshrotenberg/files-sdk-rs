//! Automation operations
//!
//! Automations enable scheduled or event-driven file operations without manual intervention.
//! Create workflows that automatically process files based on schedules, file events, or webhooks.
//!
//! # Features
//!
//! - Schedule automated file operations (copy, move, delete)
//! - Trigger actions on file events (upload, download, modify)
//! - Configure recurring tasks (daily, weekly, monthly)
//! - Set up webhook-triggered automations
//! - Manage syncs with remote servers
//! - Import files from external URLs
//!
//! # Automation Types
//!
//! - `create_folder` - Create directories automatically
//! - `delete_file` - Delete files matching patterns
//! - `copy_file` - Copy files to destinations
//! - `move_file` - Move files between locations
//! - `run_sync` - Execute sync operations
//! - `import_file` - Import from external URLs
//!
//! # Example
//!
//! ```no_run
//! use files_sdk::{FilesClient, AutomationHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = FilesClient::builder()
//!     .api_key("your-api-key")
//!     .build()?;
//!
//! let handler = AutomationHandler::new(client);
//!
//! // Create automation to copy uploaded files to archive daily
//! let automation = handler.create(
//!     "copy_file",
//!     Some("/uploads/*.pdf"),
//!     Some("/archive/"),
//!     None,
//!     Some("day"),
//!     Some("/uploads"),
//!     Some("daily")
//! ).await?;
//!
//! println!("Created automation ID: {}", automation.id.unwrap());
//!
//! // Manually trigger the automation
//! handler.manual_run(automation.id.unwrap()).await?;
//! # Ok(())
//! # }
//! ```

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Automation type enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationType {
    CreateFolder,
    DeleteFile,
    CopyFile,
    MoveFile,
    As2Send,
    RunSync,
    ImportFile,
}

/// Automation trigger type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationTrigger {
    Daily,
    Custom,
    Webhook,
    Email,
    Action,
    Interval,
}

/// An Automation entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationEntity {
    /// Automation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Force automation runs to be serialized
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_serialize_jobs: Option<bool>,

    /// Always overwrite files with matching size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always_overwrite_size_matching_files: Option<bool>,

    /// Automation type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<String>,

    /// Indicates if the automation has been deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    /// Description for this Automation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// String to replace in destination path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_replace_from: Option<String>,

    /// Replacement string for destination path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_replace_to: Option<String>,

    /// Destination paths
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<String>>,

    /// If true, this automation will not run
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    /// Glob pattern to exclude files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_pattern: Option<String>,

    /// Flatten destination folder structure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten_destination_structure: Option<bool>,

    /// Group IDs associated with automation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<i64>>,

    /// Holiday region for scheduling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holiday_region: Option<String>,

    /// Human readable schedule description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_readable_schedule: Option<String>,

    /// Ignore locked folders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_locked_folders: Option<bool>,

    /// URLs to import from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_urls: Option<Vec<serde_json::Value>>,

    /// Automation interval (day, week, month, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,

    /// Last modification time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,

    /// Use legacy folder matching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_folder_matching: Option<bool>,

    /// Legacy sync IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_sync_ids: Option<Vec<i64>>,

    /// Automation name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Overwrite existing files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite_files: Option<bool>,

    /// Path on which this Automation runs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Path timezone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_time_zone: Option<String>,

    /// Recurring day of interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_day: Option<i64>,

    /// Retry interval on failure (minutes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_on_failure_interval_in_minutes: Option<i64>,

    /// Number of retry attempts on failure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_on_failure_number_of_attempts: Option<i64>,

    /// Custom schedule configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<serde_json::Value>,

    /// Days of week for schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_days_of_week: Option<Vec<i64>>,

    /// Schedule timezone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time_zone: Option<String>,

    /// Times of day for schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_times_of_day: Option<Vec<String>>,

    /// Source path/glob
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Sync IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_ids: Option<Vec<i64>>,

    /// Trigger type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,

    /// Actions that trigger this automation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_actions: Option<Vec<String>>,

    /// User ID that owns this automation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// User IDs associated with automation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<i64>>,

    /// Automation value/configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,

    /// Webhook URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

/// Handler for automation operations
pub struct AutomationHandler {
    client: FilesClient,
}

impl AutomationHandler {
    /// Create a new automation handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List all automations
    ///
    /// Returns a paginated list of automation workflows with optional filtering
    /// by automation type.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Pagination cursor from previous response
    /// * `per_page` - Number of results per page (max 10,000)
    /// * `automation` - Filter by automation type (e.g., "copy_file", "move_file")
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - Vector of `AutomationEntity` objects
    /// - `PaginationInfo` with cursors for next/previous pages
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, AutomationHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = AutomationHandler::new(client);
    ///
    /// // List all automations
    /// let (automations, pagination) = handler.list(None, Some(50), None).await?;
    ///
    /// for automation in automations {
    ///     println!("{}: {} - Disabled: {}",
    ///         automation.name.unwrap_or_default(),
    ///         automation.automation.unwrap_or_default(),
    ///         automation.disabled.unwrap_or(false));
    /// }
    ///
    /// // Filter by type
    /// let (copy_automations, _) = handler.list(None, None, Some("copy_file")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        cursor: Option<&str>,
        per_page: Option<i64>,
        automation: Option<&str>,
    ) -> Result<(Vec<AutomationEntity>, PaginationInfo)> {
        let mut params = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        if let Some(pp) = per_page {
            params.push(("per_page", pp.to_string()));
        }
        if let Some(a) = automation {
            params.push(("automation", a.to_string()));
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
            .get_raw(&format!("/automations{}", query))
            .await?;
        let automations: Vec<AutomationEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((automations, pagination))
    }

    /// Get details of a specific automation
    ///
    /// # Arguments
    ///
    /// * `id` - Automation ID
    ///
    /// # Returns
    ///
    /// An `AutomationEntity` with complete automation configuration
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, AutomationHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = AutomationHandler::new(client);
    ///
    /// let automation = handler.get(12345).await?;
    /// println!("Automation: {}", automation.name.unwrap_or_default());
    /// println!("Schedule: {}", automation.human_readable_schedule.unwrap_or_default());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, id: i64) -> Result<AutomationEntity> {
        let response = self.client.get_raw(&format!("/automations/{}", id)).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new automation workflow
    ///
    /// Creates an automation that performs file operations automatically based on
    /// schedules or triggers.
    ///
    /// # Arguments
    ///
    /// * `automation` - Type of automation: "copy_file", "move_file", "delete_file",
    ///   "create_folder", "run_sync", "import_file" (required)
    /// * `source` - Source path or glob pattern (e.g., "/uploads/*.pdf")
    /// * `destination` - Single destination path
    /// * `destinations` - Multiple destination paths (use instead of destination)
    /// * `interval` - Schedule interval: "day", "week", "month", "year"
    /// * `path` - Base path where automation operates
    /// * `trigger` - Trigger type: "daily", "custom", "webhook", "email", "action", "interval"
    ///
    /// # Returns
    ///
    /// The newly created `AutomationEntity`
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, AutomationHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = AutomationHandler::new(client);
    ///
    /// // Create daily automation to archive PDFs
    /// let automation = handler.create(
    ///     "copy_file",
    ///     Some("/uploads/*.pdf"),
    ///     Some("/archive/daily/"),
    ///     None,
    ///     Some("day"),
    ///     Some("/uploads"),
    ///     Some("daily")
    /// ).await?;
    ///
    /// println!("Created automation: {}", automation.id.unwrap());
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        automation: &str,
        source: Option<&str>,
        destination: Option<&str>,
        destinations: Option<Vec<String>>,
        interval: Option<&str>,
        path: Option<&str>,
        trigger: Option<&str>,
    ) -> Result<AutomationEntity> {
        let mut request_body = json!({
            "automation": automation,
        });

        if let Some(s) = source {
            request_body["source"] = json!(s);
        }
        if let Some(d) = destination {
            request_body["destination"] = json!(d);
        }
        if let Some(dests) = destinations {
            request_body["destinations"] = json!(dests);
        }
        if let Some(i) = interval {
            request_body["interval"] = json!(i);
        }
        if let Some(p) = path {
            request_body["path"] = json!(p);
        }
        if let Some(t) = trigger {
            request_body["trigger"] = json!(t);
        }

        let response = self.client.post_raw("/automations", request_body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update an existing automation
    ///
    /// Modifies automation configuration. Only provided fields are updated;
    /// omitted fields remain unchanged.
    ///
    /// # Arguments
    ///
    /// * `id` - Automation ID to update
    /// * `source` - New source path or glob pattern
    /// * `destination` - New destination path
    /// * `interval` - New schedule interval
    /// * `disabled` - Enable (false) or disable (true) the automation
    ///
    /// # Returns
    ///
    /// The updated `AutomationEntity`
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, AutomationHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = AutomationHandler::new(client);
    ///
    /// // Disable an automation temporarily
    /// let automation = handler.update(
    ///     12345,
    ///     None,
    ///     None,
    ///     None,
    ///     Some(true)
    /// ).await?;
    ///
    /// println!("Automation disabled");
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        id: i64,
        source: Option<&str>,
        destination: Option<&str>,
        interval: Option<&str>,
        disabled: Option<bool>,
    ) -> Result<AutomationEntity> {
        let mut request_body = json!({});

        if let Some(s) = source {
            request_body["source"] = json!(s);
        }
        if let Some(d) = destination {
            request_body["destination"] = json!(d);
        }
        if let Some(i) = interval {
            request_body["interval"] = json!(i);
        }
        if let Some(dis) = disabled {
            request_body["disabled"] = json!(dis);
        }

        let response = self
            .client
            .patch_raw(&format!("/automations/{}", id), request_body)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete an automation permanently
    ///
    /// Removes the automation and stops all scheduled or triggered executions.
    /// This operation cannot be undone.
    ///
    /// # Arguments
    ///
    /// * `id` - Automation ID to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, AutomationHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = AutomationHandler::new(client);
    ///
    /// handler.delete(12345).await?;
    /// println!("Automation deleted");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, id: i64) -> Result<()> {
        self.client
            .delete_raw(&format!("/automations/{}", id))
            .await?;
        Ok(())
    }

    /// Manually trigger an automation execution
    ///
    /// Immediately executes the automation regardless of its schedule or trigger settings.
    /// Useful for testing or running an automation on-demand.
    ///
    /// # Arguments
    ///
    /// * `id` - Automation ID to execute
    ///
    /// # Returns
    ///
    /// JSON response with execution details and status
    ///
    /// # Example
    ///
    /// ```no_run
    /// use files_sdk::{FilesClient, AutomationHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = AutomationHandler::new(client);
    ///
    /// // Manually trigger the automation to run now
    /// let result = handler.manual_run(12345).await?;
    /// println!("Automation executed: {:?}", result);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn manual_run(&self, id: i64) -> Result<serde_json::Value> {
        let response = self
            .client
            .post_raw(&format!("/automations/{}/manual_run", id), json!({}))
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = AutomationHandler::new(client);
    }
}
