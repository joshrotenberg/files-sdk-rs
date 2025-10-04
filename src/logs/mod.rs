//! Logging and audit module
//!
//! This module contains handlers for various logs and audit trails:
//! - API request logs
//! - SFTP, FTP, WebDAV action logs
//! - Automation and sync logs
//! - Email logs
//! - Outbound connection logs
//! - Settings changes
//! - Public hosting request logs

pub mod api_request_logs;
pub mod automation_logs;
pub mod email_logs;
pub mod exavault_api_request_logs;
pub mod ftp_action_logs;
pub mod outbound_connection_logs;
pub mod public_hosting_request_logs;
pub mod settings_changes;
pub mod sftp_action_logs;
pub mod sync_logs;
pub mod web_dav_action_logs;

// Re-export handlers
pub use api_request_logs::ApiRequestLogHandler;
pub use automation_logs::AutomationLogHandler;
pub use email_logs::EmailLogHandler;
pub use exavault_api_request_logs::ExavaultApiRequestLogHandler;
pub use ftp_action_logs::FtpActionLogHandler;
pub use outbound_connection_logs::OutboundConnectionLogHandler;
pub use public_hosting_request_logs::PublicHostingRequestLogHandler;
pub use settings_changes::SettingsChangeHandler;
pub use sftp_action_logs::SftpActionLogHandler;
pub use sync_logs::SyncLogHandler;
pub use web_dav_action_logs::WebDavActionLogHandler;

// Re-export entities
pub use api_request_logs::ApiRequestLogEntity;
pub use automation_logs::AutomationLogEntity;
pub use email_logs::EmailLogEntity;
pub use exavault_api_request_logs::ExavaultApiRequestLogEntity;
pub use ftp_action_logs::FtpActionLogEntity;
pub use outbound_connection_logs::OutboundConnectionLogEntity;
pub use public_hosting_request_logs::PublicHostingRequestLogEntity;
pub use settings_changes::SettingsChangeEntity;
pub use sftp_action_logs::SftpActionLogEntity;
pub use sync_logs::SyncLogEntity;
pub use web_dav_action_logs::WebDavActionLogEntity;
