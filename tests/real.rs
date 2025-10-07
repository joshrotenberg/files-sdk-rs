//! Real API integration tests
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests --test real
//!
//! Tests create and clean up resources in /integration-tests/ folder.

#![cfg(feature = "integration-tests")]

// Common utilities
#[path = "real/mod.rs"]
mod real;

// File operations tests
#[path = "real/files/mod.rs"]
mod files;

// User management tests
#[path = "real/users/mod.rs"]
mod users;

// Sharing features tests
#[path = "real/sharing/mod.rs"]
mod sharing;

// Admin features tests
#[path = "real/admin/mod.rs"]
mod admin;

// Automation features tests
#[path = "real/automation/mod.rs"]
mod automation;

// Storage management tests
#[path = "real/storage/mod.rs"]
mod storage;

// Security features tests
#[path = "real/security/mod.rs"]
mod security;

// Logging and audit tests
#[path = "real/logs/mod.rs"]
mod logs;

// Messaging tests
#[path = "real/messages/mod.rs"]
mod messages;

// AS2 protocol tests
#[path = "real/as2/mod.rs"]
mod as2;

// Developer resources tests
#[path = "real/developers/mod.rs"]
mod developers;

// Integration tests
#[path = "real/integrations/mod.rs"]
mod integrations;
