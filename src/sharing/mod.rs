//! File sharing and collaboration module
//!
//! This module contains handlers for sharing files and managing collaborations:
//! - Bundle creation and management (share links)
//! - Bundle recipients and notifications
//! - File requests
//! - Inbox uploads and recipients

pub mod bundle_actions;
pub mod bundle_downloads;
pub mod bundle_notifications;
pub mod bundle_recipients;
pub mod bundle_registrations;
pub mod bundles;
pub mod form_field_sets;
pub mod inbox_recipients;
pub mod inbox_registrations;
pub mod inbox_uploads;
pub mod requests;
pub mod share_groups;

// Re-export handlers
pub use bundle_actions::BundleActionHandler;
pub use bundle_downloads::BundleDownloadHandler;
pub use bundle_notifications::BundleNotificationHandler;
pub use bundle_recipients::BundleRecipientHandler;
pub use bundle_registrations::BundleRegistrationHandler;
pub use bundles::BundleHandler;
pub use form_field_sets::FormFieldSetHandler;
pub use inbox_recipients::InboxRecipientHandler;
pub use inbox_registrations::InboxRegistrationHandler2;
pub use inbox_uploads::InboxUploadHandler;
pub use requests::RequestHandler;
pub use share_groups::ShareGroupHandler;

// Re-export entities
pub use bundle_actions::BundleActionEntity;
pub use bundle_downloads::BundleDownloadEntity;
pub use bundle_notifications::BundleNotificationEntity;
pub use bundle_recipients::BundleRecipientEntity;
pub use bundle_registrations::BundleRegistrationEntity;
pub use bundles::{BundleEntity, BundlePermission};
pub use form_field_sets::FormFieldSetEntity;
pub use inbox_recipients::InboxRecipientEntity;
pub use inbox_registrations::InboxRegistrationEntity2;
pub use inbox_uploads::{InboxRegistrationEntity, InboxUploadEntity};
pub use requests::RequestEntity;
pub use share_groups::ShareGroupEntity;
