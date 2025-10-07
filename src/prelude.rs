//! Convenient re-exports for common types and traits
//!
//! This module provides a convenient way to import the most commonly used types
//! and traits from the Files.com SDK. It follows the Rust convention of providing
//! a `prelude` module for frequently-used items.
//!
//! # Usage
//!
//! ```rust
//! use files_sdk::prelude::*;
//!
//! // Now you have access to all common types without individual imports
//! # fn example() -> Result<()> {
//! let client = FilesClient::builder()
//!     .api_key("your-api-key")
//!     .build()?;
//!
//! let file_handler = FileHandler::new(client.clone());
//! # Ok(())
//! # }
//! ```

// Core client and error types
pub use crate::client::{FilesClient, FilesClientBuilder};
pub use crate::error::{FilesError, Result};

// Common entity types
pub use crate::types::{FileEntity, FileUploadPartEntity, FolderEntity, PaginationInfo};

// Progress tracking
pub use crate::progress::{Progress, ProgressCallback};

// Most commonly used handlers
pub use crate::files::{FileActionHandler, FileHandler, FolderHandler};
pub use crate::users::{ApiKeyHandler, GroupHandler, SessionHandler, UserHandler};

// Sharing
pub use crate::sharing::{BundleHandler, RequestHandler};

// Automation
pub use crate::automation::{AutomationHandler, BehaviorHandler};
