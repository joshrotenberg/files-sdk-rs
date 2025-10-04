//! Files.com Rust SDK
//!
//! A comprehensive Rust client for the [Files.com](https://files.com) REST API, providing full access to
//! file operations, user management, sharing, automation, and administrative features.
//!
//! ## Features
//!
//! - **File Operations**: Upload, download, copy, move, delete files and folders
//! - **User & Access Management**: Users, groups, permissions, API keys, sessions
//! - **Sharing**: Bundles (share links), file requests, inbox uploads
//! - **Automation**: Webhooks, behaviors, remote servers, automations
//! - **Administration**: Site settings, history, notifications, billing
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use files_sdk::{FilesClient, FileHandler};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client with API key
//!     let client = FilesClient::builder()
//!         .api_key("your-api-key")
//!         .build()?;
//!
//!     // Use handlers for typed operations
//!     let file_handler = FileHandler::new(client.clone());
//!
//!     // Upload a file
//!     let data = b"Hello, Files.com!";
//!     file_handler.upload_file("/path/to/file.txt", data).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Core Usage Patterns
//!
//! ### Client Creation
//!
//! The client uses a builder pattern for flexible configuration:
//!
//! ```rust,no_run
//! use files_sdk::FilesClient;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Basic client with default settings
//! let client = FilesClient::builder()
//!     .api_key("your-api-key")
//!     .build()?;
//!
//! // Custom configuration
//! let client = FilesClient::builder()
//!     .api_key("your-api-key")
//!     .base_url("https://app.files.com/api/rest/v1".to_string())
//!     .timeout(std::time::Duration::from_secs(60))
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ### File Upload (Two-Stage Process)
//!
//! Files.com uses a two-stage upload process:
//!
//! ```rust,no_run
//! use files_sdk::{FilesClient, FileActionHandler, FileHandler};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = FilesClient::builder()
//!     .api_key("your-api-key")
//!     .build()?;
//!
//! // Stage 1: Begin upload to get upload URLs
//! let file_action = FileActionHandler::new(client.clone());
//! let upload_info = file_action
//!     .begin_upload("/uploads/myfile.txt", Some(1024), true)
//!     .await?;
//!
//! // Stage 2: Upload file data (simplified - see FileHandler for complete implementation)
//! let file_handler = FileHandler::new(client.clone());
//! let data = b"file contents";
//! file_handler.upload_file("/uploads/myfile.txt", data).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! The SDK provides comprehensive error handling:
//!
//! ```rust,no_run
//! use files_sdk::{FilesClient, FilesError, FileHandler};
//!
//! # #[tokio::main]
//! # async fn main() {
//! let client = FilesClient::builder()
//!     .api_key("test-key")
//!     .build()
//!     .unwrap();
//!
//! let handler = FileHandler::new(client);
//!
//! match handler.download_file("/path/to/file.txt").await {
//!     Ok(file) => println!("Downloaded: {:?}", file),
//!     Err(FilesError::NotFound { message }) => {
//!         println!("File not found: {}", message);
//!     }
//!     Err(FilesError::AuthenticationFailed { message }) => {
//!         println!("Invalid API key: {}", message);
//!     }
//!     Err(e) => println!("Other error: {}", e),
//! }
//! # }
//! ```
//!
//! ## Authentication
//!
//! Files.com uses API key authentication via the `X-FilesAPI-Key` header.
//! API keys can be obtained from the Files.com web interface under Account Settings.

pub mod api_keys;
pub mod automations;
pub mod behaviors;
pub mod bundles;
pub mod client;
pub mod file_actions;
pub mod file_comments;
pub mod files;
pub mod folders;
pub mod group_users;
pub mod groups;
pub mod history;
pub mod inbox_uploads;
pub mod invoices;
pub mod locks;
pub mod messages;
pub mod notifications;
pub mod payments;
pub mod permissions;
pub mod projects;
pub mod public_keys;
pub mod remote_servers;
pub mod requests;
pub mod sessions;
pub mod site;
pub mod types;
pub mod users;
// Advanced handlers
pub mod action_notification_export_results;
pub mod action_notification_exports;
pub mod api_key;
pub mod apps;
pub mod as2_incoming_messages;
pub mod as2_outgoing_messages;
pub mod as2_partners;
pub mod as2_stations;
pub mod automation_runs;
pub mod bandwidth_snapshots;
pub mod bundle_actions;
pub mod bundle_downloads;
pub mod bundle_notifications;
pub mod bundle_recipients;
pub mod bundle_registrations;
pub mod child_site_management_policies;
pub mod clickwraps;
pub mod dns_records;
pub mod email_incoming_messages;
pub mod exavault_api_request_logs;
pub mod external_events;
pub mod file_comment_reactions;
pub mod file_migrations;
pub mod form_field_sets;
pub mod gpg_keys;
pub mod history_export_results;
pub mod history_exports;
pub mod holiday_regions;
pub mod inbox_recipients;
pub mod inbox_registrations;
pub mod ip_addresses;
pub mod message_comment_reactions;
pub mod message_comments;
pub mod message_reactions;
pub mod priorities;
pub mod public_hosting_request_logs;
pub mod remote_bandwidth_snapshots;
pub mod remote_mount_backends;
pub mod restores;
pub mod sftp_host_keys;
pub mod share_groups;
pub mod siem_http_destinations;
pub mod snapshots;
pub mod sso_strategies;
pub mod styles;
pub mod sync_runs;
pub mod syncs;
pub mod usage_daily_snapshots;
pub mod usage_snapshots;
pub mod user;
pub mod user_cipher_uses;
pub mod user_lifecycle_rules;
pub mod user_requests;
pub mod user_sftp_client_uses;
pub mod webhook_tests;

// Log and monitoring handlers
pub mod api_request_logs;
pub mod automation_logs;
pub mod email_logs;
pub mod file_migration_logs;
pub mod ftp_action_logs;
pub mod outbound_connection_logs;
pub mod settings_changes;
pub mod sftp_action_logs;
pub mod sync_logs;
pub mod web_dav_action_logs;

// Re-export client types
pub use client::{FilesClient, FilesClientBuilder};

// Re-export handlers
pub use api_keys::ApiKeyHandler;
pub use automations::AutomationHandler;
pub use behaviors::BehaviorHandler;
pub use bundles::BundleHandler;
pub use file_actions::FileActionHandler;
pub use file_comments::FileCommentHandler;
pub use files::FileHandler;
pub use folders::FolderHandler;
pub use group_users::GroupUserHandler;
pub use groups::GroupHandler;
pub use history::HistoryHandler;
pub use inbox_uploads::InboxUploadHandler;
pub use invoices::InvoiceHandler;
pub use locks::LockHandler;
pub use messages::MessageHandler;
pub use notifications::NotificationHandler;
pub use payments::PaymentHandler;
pub use permissions::PermissionHandler;
pub use projects::ProjectHandler;
pub use public_keys::PublicKeyHandler;
pub use remote_servers::RemoteServerHandler;
pub use requests::RequestHandler;
pub use sessions::SessionHandler;
pub use site::SiteHandler;

// Log and monitoring handlers

// Re-export common types
pub use api_keys::ApiKeyEntity;
pub use automations::AutomationEntity;
pub use behaviors::BehaviorEntity;
pub use bundles::BundleEntity;
pub use file_comments::FileCommentEntity;
pub use group_users::GroupUserEntity;
pub use groups::GroupEntity;
pub use history::{HistoryExportEntity, HistoryExportResultEntity};
pub use inbox_uploads::{InboxRegistrationEntity, InboxUploadEntity};
pub use invoices::{AccountLineItemEntity, InvoiceLineItemEntity};
pub use locks::LockEntity;
pub use messages::MessageEntity;
pub use notifications::NotificationEntity;
pub use payments::{PaymentEntity, PaymentLineItemEntity};
pub use permissions::PermissionEntity;
pub use projects::ProjectEntity;
pub use public_keys::PublicKeyEntity;
pub use remote_servers::RemoteServerEntity;
pub use requests::RequestEntity;
pub use sessions::SessionEntity;
pub use site::{SiteEntity, SiteUsageEntity};
pub use types::{FileEntity, FileUploadPartEntity, FolderEntity, PaginationInfo};

// Log and monitoring entities

// Error handling
use thiserror::Error;

/// Errors that can occur when using the Files.com API
#[derive(Error, Debug)]
pub enum FilesError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// Bad Request (400)
    #[error("Bad Request (400): {message}")]
    BadRequest { message: String },

    /// Authentication failed (401)
    #[error("Authentication failed (401): {message}")]
    AuthenticationFailed { message: String },

    /// Forbidden (403)
    #[error("Forbidden (403): {message}")]
    Forbidden { message: String },

    /// Not Found (404)
    #[error("Not Found (404): {message}")]
    NotFound { message: String },

    /// Rate Limited (429)
    #[error("Rate Limited (429): {message}")]
    RateLimited { message: String },

    /// Internal Server Error (500)
    #[error("Internal Server Error (500): {message}")]
    InternalServerError { message: String },

    /// Service Unavailable (503)
    #[error("Service Unavailable (503): {message}")]
    ServiceUnavailable { message: String },

    /// Generic API error with status code
    #[error("API error ({code}): {message}")]
    ApiError { code: u16, message: String },

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Result type for Files.com operations
pub type Result<T> = std::result::Result<T, FilesError>;
// Advanced entities
pub use action_notification_export_results::ActionNotificationExportResultEntity;
pub use action_notification_exports::ActionNotificationExportEntity;
pub use api_key::ApiKeyCurrentEntity;
pub use apps::AppEntity;
pub use as2_incoming_messages::As2IncomingMessageEntity;
pub use as2_outgoing_messages::As2OutgoingMessageEntity;
pub use as2_partners::As2PartnerEntity;
pub use as2_stations::As2StationEntity;
pub use automation_runs::AutomationRunEntity;
pub use bandwidth_snapshots::BandwidthSnapshotEntity;
pub use bundle_actions::BundleActionEntity;
pub use bundle_downloads::BundleDownloadEntity;
pub use bundle_notifications::BundleNotificationEntity;
pub use bundle_recipients::BundleRecipientEntity;
pub use bundle_registrations::BundleRegistrationEntity;
pub use child_site_management_policies::ChildSiteManagementPolicyEntity;
pub use clickwraps::ClickwrapEntity;
pub use dns_records::DnsRecordEntity;
pub use email_incoming_messages::EmailIncomingMessageEntity;
pub use exavault_api_request_logs::ExavaultApiRequestLogEntity;
pub use external_events::ExternalEventEntity;
pub use file_comment_reactions::FileCommentReactionEntity;
pub use file_migrations::FileMigrationEntity;
pub use form_field_sets::FormFieldSetEntity;
pub use gpg_keys::GpgKeyEntity;
pub use history_export_results::HistoryExportResultEntity2;
pub use history_exports::HistoryExportEntity2;
pub use holiday_regions::HolidayRegionEntity;
pub use inbox_recipients::InboxRecipientEntity;
pub use inbox_registrations::InboxRegistrationEntity2;
pub use ip_addresses::IpAddressEntity;
pub use message_comment_reactions::MessageCommentReactionEntity;
pub use message_comments::MessageCommentEntity;
pub use message_reactions::MessageReactionEntity;
pub use priorities::PriorityEntity;
pub use public_hosting_request_logs::PublicHostingRequestLogEntity;
pub use remote_bandwidth_snapshots::RemoteBandwidthSnapshotEntity;
pub use remote_mount_backends::RemoteMountBackendEntity;
pub use restores::RestoreEntity;
pub use sftp_host_keys::SftpHostKeyEntity;
pub use share_groups::ShareGroupEntity;
pub use siem_http_destinations::SiemHttpDestinationEntity;
pub use snapshots::SnapshotEntity;
pub use sso_strategies::SsoStrategyEntity;
pub use styles::StyleEntity;
pub use sync_runs::SyncRunEntity;
pub use syncs::SyncEntity;
pub use usage_daily_snapshots::UsageDailySnapshotEntity;
pub use usage_snapshots::UsageSnapshotEntity;
pub use user_cipher_uses::UserCipherUseEntity;
pub use user_lifecycle_rules::UserLifecycleRuleEntity;
pub use user_requests::UserRequestEntity;
pub use user_sftp_client_uses::UserSftpClientUseEntity;
pub use webhook_tests::WebhookTestEntity;

pub use api_request_logs::ApiRequestLogEntity;
// Advanced handlers
pub use action_notification_export_results::ActionNotificationExportResultHandler;
pub use action_notification_exports::ActionNotificationExportHandler;
pub use api_key::ApiKeyCurrentHandler;
pub use apps::AppHandler;
pub use as2_incoming_messages::As2IncomingMessageHandler;
pub use as2_outgoing_messages::As2OutgoingMessageHandler;
pub use as2_partners::As2PartnerHandler;
pub use as2_stations::As2StationHandler;
pub use automation_runs::AutomationRunHandler;
pub use bandwidth_snapshots::BandwidthSnapshotHandler;
pub use bundle_actions::BundleActionHandler;
pub use bundle_downloads::BundleDownloadHandler;
pub use bundle_notifications::BundleNotificationHandler;
pub use bundle_recipients::BundleRecipientHandler;
pub use bundle_registrations::BundleRegistrationHandler;
pub use child_site_management_policies::ChildSiteManagementPolicyHandler;
pub use clickwraps::ClickwrapHandler;
pub use dns_records::DnsRecordHandler;
pub use email_incoming_messages::EmailIncomingMessageHandler;
pub use exavault_api_request_logs::ExavaultApiRequestLogHandler;
pub use external_events::ExternalEventHandler;
pub use file_comment_reactions::FileCommentReactionHandler;
pub use file_migrations::FileMigrationHandler;
pub use form_field_sets::FormFieldSetHandler;
pub use gpg_keys::GpgKeyHandler;
pub use history_export_results::HistoryExportResultHandler2;
pub use history_exports::HistoryExportHandler2;
pub use holiday_regions::HolidayRegionHandler;
pub use inbox_recipients::InboxRecipientHandler;
pub use inbox_registrations::InboxRegistrationHandler2;
pub use ip_addresses::IpAddressHandler;
pub use message_comment_reactions::MessageCommentReactionHandler;
pub use message_comments::MessageCommentHandler;
pub use message_reactions::MessageReactionHandler;
pub use priorities::PriorityHandler;
pub use public_hosting_request_logs::PublicHostingRequestLogHandler;
pub use remote_bandwidth_snapshots::RemoteBandwidthSnapshotHandler;
pub use remote_mount_backends::RemoteMountBackendHandler;
pub use restores::RestoreHandler;
pub use sftp_host_keys::SftpHostKeyHandler;
pub use share_groups::ShareGroupHandler;
pub use siem_http_destinations::SiemHttpDestinationHandler;
pub use snapshots::SnapshotHandler;
pub use sso_strategies::SsoStrategyHandler;
pub use styles::StyleHandler;
pub use sync_runs::SyncRunHandler;
pub use syncs::SyncHandler;
pub use usage_daily_snapshots::UsageDailySnapshotHandler;
pub use usage_snapshots::UsageSnapshotHandler;
pub use user::CurrentUserHandler;
pub use user_cipher_uses::UserCipherUseHandler;
pub use user_lifecycle_rules::UserLifecycleRuleHandler;
pub use user_requests::UserRequestHandler;
pub use user_sftp_client_uses::UserSftpClientUseHandler;
pub use users::UserHandler;
pub use webhook_tests::WebhookTestHandler;

pub use api_request_logs::ApiRequestLogHandler;
pub use automation_logs::AutomationLogEntity;
pub use automation_logs::AutomationLogHandler;
pub use email_logs::EmailLogEntity;
pub use email_logs::EmailLogHandler;
pub use file_migration_logs::FileMigrationLogEntity;
pub use file_migration_logs::FileMigrationLogHandler;
pub use ftp_action_logs::FtpActionLogEntity;
pub use ftp_action_logs::FtpActionLogHandler;
pub use outbound_connection_logs::OutboundConnectionLogEntity;
pub use outbound_connection_logs::OutboundConnectionLogHandler;
pub use settings_changes::SettingsChangeEntity;
pub use settings_changes::SettingsChangeHandler;
pub use sftp_action_logs::SftpActionLogEntity;
pub use sftp_action_logs::SftpActionLogHandler;
pub use sync_logs::SyncLogEntity;
pub use sync_logs::SyncLogHandler;
pub use web_dav_action_logs::WebDavActionLogEntity;
pub use web_dav_action_logs::WebDavActionLogHandler;
