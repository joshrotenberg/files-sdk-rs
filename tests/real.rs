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
