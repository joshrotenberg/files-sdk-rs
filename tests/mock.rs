//! Mock-based integration tests
//!
//! These tests use wiremock to simulate the Files.com API and test
//! the SDK's behavior without requiring a real API connection.
//!
//! Run with: cargo test --test mock

// Common utilities
#[path = "mock/mod.rs"]
mod mock;

// File operations tests
#[path = "mock/files/mod.rs"]
mod files;

// User and access management tests
#[path = "mock/users/mod.rs"]
mod users;

// Sharing and collaboration tests
#[path = "mock/sharing/mod.rs"]
mod sharing;

// Automation and integration tests
#[path = "mock/automation/mod.rs"]
mod automation;

// Messaging and notifications tests
#[path = "mock/messages/mod.rs"]
mod messages;

// Admin and monitoring tests
#[path = "mock/admin/mod.rs"]
mod admin;
