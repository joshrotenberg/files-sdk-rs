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
pub mod bundles;
pub mod client;
pub mod file_actions;
pub mod files;
pub mod folders;
pub mod groups;
pub mod permissions;
pub mod sessions;
pub mod types;
pub mod users;

// Re-export client types
pub use client::{FilesClient, FilesClientBuilder};

// Re-export handlers
pub use api_keys::ApiKeyHandler;
pub use bundles::BundleHandler;
pub use file_actions::FileActionHandler;
pub use files::FileHandler;
pub use folders::FolderHandler;
pub use groups::GroupHandler;
pub use permissions::PermissionHandler;
pub use sessions::SessionHandler;
pub use users::UserHandler;

// Re-export common types
pub use api_keys::ApiKeyEntity;
pub use bundles::BundleEntity;
pub use groups::GroupEntity;
pub use permissions::PermissionEntity;
pub use sessions::SessionEntity;
pub use types::{FileEntity, FileUploadPartEntity, FolderEntity, PaginationInfo};
pub use users::UserEntity;

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
