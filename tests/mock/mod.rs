//! Common utilities for mock tests
//!
//! This module provides shared functionality for all mock-based tests,
//! including mock server setup and common test patterns.

// Re-export wiremock items for use in test modules
pub use wiremock::matchers::{header, method, path, query_param};
pub use wiremock::{Mock, MockServer, ResponseTemplate};

use files_sdk::FilesClient;

/// Creates a test client configured for the given mock server
pub fn create_test_client(mock_server: &MockServer) -> FilesClient {
    FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap()
}

/// Creates a standard 404 Not Found error response
pub fn not_found_response() -> ResponseTemplate {
    let error_body = serde_json::json!({
        "error": "Not Found",
        "message": "Resource not found"
    });
    ResponseTemplate::new(404).set_body_json(&error_body)
}

/// Creates a standard 400 Bad Request error response
pub fn bad_request_response(message: &str) -> ResponseTemplate {
    let error_body = serde_json::json!({
        "error": "Bad Request",
        "message": message
    });
    ResponseTemplate::new(400).set_body_json(&error_body)
}

/// Creates a standard 403 Forbidden error response
pub fn forbidden_response(message: &str) -> ResponseTemplate {
    let error_body = serde_json::json!({
        "error": "Forbidden",
        "message": message
    });
    ResponseTemplate::new(403).set_body_json(&error_body)
}

/// Creates a standard 401 Unauthorized error response
pub fn unauthorized_response() -> ResponseTemplate {
    let error_body = serde_json::json!({
        "error": "Unauthorized",
        "message": "Authentication failed"
    });
    ResponseTemplate::new(401).set_body_json(&error_body)
}
