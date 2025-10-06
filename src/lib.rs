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
//! use files_sdk::{FilesClient, files::FileHandler};
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
//! use files_sdk::{FilesClient, files::{FileActionHandler, FileHandler}};
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
//! use files_sdk::{FilesClient, FilesError, files::FileHandler};
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
//!     Err(FilesError::NotFound { message, .. }) => {
//!         println!("File not found: {}", message);
//!     }
//!     Err(FilesError::AuthenticationFailed { message, .. }) => {
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

// Core modules
pub mod client;
pub mod error;
pub mod types;
pub mod utils;

// Domain modules
pub mod admin;
pub mod as2;
pub mod automation;
pub mod developers;
pub mod files;
pub mod integrations;
pub mod logs;
pub mod messages;
pub mod security;
pub mod sharing;
pub mod storage;
pub mod users;

// Misc (to be moved or removed)
pub mod webhook_tests;

// Re-export client types
pub use client::{FilesClient, FilesClientBuilder};

// Re-export error types
pub use error::{FilesError, Result};

// Re-export common types
pub use types::{FileEntity, FileUploadPartEntity, FolderEntity, PaginationInfo};

// Re-export all handlers for backward compatibility
pub use admin::{
    ActionNotificationExportHandler, ActionNotificationExportResultHandler,
    ChildSiteManagementPolicyHandler, DnsRecordHandler, HistoryExportHandler2,
    HistoryExportResultHandler2, HistoryHandler, HolidayRegionHandler, InvoiceHandler,
    PaymentHandler, SiteHandler, StyleHandler,
};
pub use as2::{
    As2IncomingMessageHandler, As2OutgoingMessageHandler, As2PartnerHandler, As2StationHandler,
};
pub use automation::{
    AutomationHandler, AutomationRunHandler, BehaviorHandler, RemoteMountBackendHandler,
    RemoteServerHandler, SyncHandler, SyncRunHandler,
};
pub use developers::AppHandler;
pub use files::{
    FileActionHandler, FileCommentHandler, FileCommentReactionHandler, FileHandler,
    FileMigrationHandler, FileMigrationLogHandler, FolderHandler,
};
pub use integrations::SiemHttpDestinationHandler;
pub use logs::{
    ApiRequestLogHandler, AutomationLogHandler, EmailIncomingMessageHandler, EmailLogHandler,
    ExavaultApiRequestLogHandler, ExternalEventHandler, FtpActionLogHandler,
    OutboundConnectionLogHandler, PublicHostingRequestLogHandler, SettingsChangeHandler,
    SftpActionLogHandler, SyncLogHandler, WebDavActionLogHandler,
};
pub use messages::{
    MessageCommentHandler, MessageCommentReactionHandler, MessageHandler, MessageReactionHandler,
    NotificationHandler,
};
pub use security::{ClickwrapHandler, GpgKeyHandler, IpAddressHandler, SftpHostKeyHandler};
pub use sharing::{
    BundleActionHandler, BundleDownloadHandler, BundleHandler, BundleNotificationHandler,
    BundleRecipientHandler, BundleRegistrationHandler, FormFieldSetHandler, InboxRecipientHandler,
    InboxRegistrationHandler2, InboxUploadHandler, RequestHandler, ShareGroupHandler,
};
pub use storage::{
    BandwidthSnapshotHandler, LockHandler, PriorityHandler, ProjectHandler,
    RemoteBandwidthSnapshotHandler, RestoreHandler, SnapshotHandler, UsageDailySnapshotHandler,
    UsageSnapshotHandler,
};
pub use users::{
    ApiKeyCurrentHandler, ApiKeyHandler, CurrentUserHandler, GroupHandler, GroupUserHandler,
    PermissionHandler, PublicKeyHandler, SessionHandler, SsoStrategyHandler, UserCipherUseHandler,
    UserHandler, UserLifecycleRuleHandler, UserRequestHandler, UserSftpClientUseHandler,
};

// Error types are now in the error module
