//! Security and compliance module
//!
//! This module contains handlers for security and compliance features:
//! - GPG keys
//! - Clickwraps (legal agreements)
//! - SFTP host keys
//! - IP address management

pub mod clickwraps;
pub mod gpg_keys;
pub mod ip_addresses;
pub mod sftp_host_keys;

// Re-export handlers
pub use clickwraps::ClickwrapHandler;
pub use gpg_keys::GpgKeyHandler;
pub use ip_addresses::IpAddressHandler;
pub use sftp_host_keys::SftpHostKeyHandler;

// Re-export entities
pub use clickwraps::ClickwrapEntity;
pub use gpg_keys::GpgKeyEntity;
pub use ip_addresses::IpAddressEntity;
pub use sftp_host_keys::SftpHostKeyEntity;
