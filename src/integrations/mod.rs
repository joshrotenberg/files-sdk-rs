//! Integrations module
//!
//! This module contains handlers for third-party integrations:
//! - SIEM integrations (Splunk, Sentinel, etc.)

pub mod siem_http_destinations;

// Re-export handlers
pub use siem_http_destinations::SiemHttpDestinationHandler;

// Re-export entities
pub use siem_http_destinations::SiemHttpDestinationEntity;
