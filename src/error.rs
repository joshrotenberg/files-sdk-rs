//! Error types for the Files.com SDK
//!
//! This module provides comprehensive error handling with contextual information
//! to make debugging and error handling easier.

use thiserror::Error;

/// Errors that can occur when using the Files.com API
///
/// Each error variant includes contextual information to help with debugging
/// and provide meaningful error messages to users.
#[derive(Error, Debug)]
pub enum FilesError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// Bad Request (400) - Invalid parameters or malformed request
    #[error("Bad Request (400): {message}")]
    BadRequest {
        message: String,
        /// Optional field that caused the error
        field: Option<String>,
    },

    /// Authentication failed (401)
    #[error("Authentication failed (401): {message}")]
    AuthenticationFailed {
        message: String,
        /// The authentication method that failed
        auth_type: Option<String>,
    },

    /// Forbidden (403) - Valid credentials but insufficient permissions
    #[error("Forbidden (403): {message}")]
    Forbidden {
        message: String,
        /// The resource that was forbidden
        resource: Option<String>,
    },

    /// Not Found (404) - Resource does not exist
    #[error("Not Found (404): {message}")]
    NotFound {
        message: String,
        /// Type of resource (e.g., "file", "folder", "user")
        resource_type: Option<String>,
        /// Path or identifier of the resource
        path: Option<String>,
    },

    /// Conflict (409) - Resource already exists or state conflict
    #[error("Conflict (409): {message}")]
    Conflict {
        message: String,
        /// The conflicting resource path or identifier
        resource: Option<String>,
    },

    /// Precondition Failed (412) - Conditional request failed
    #[error("Precondition Failed (412): {message}")]
    PreconditionFailed {
        message: String,
        /// The condition that failed
        condition: Option<String>,
    },

    /// Unprocessable Entity (422) - Validation error
    #[error("Unprocessable Entity (422): {message}")]
    UnprocessableEntity {
        message: String,
        /// Field that failed validation
        field: Option<String>,
        /// The invalid value provided
        value: Option<String>,
    },

    /// Locked (423) - Resource is locked
    #[error("Locked (423): {message}")]
    Locked {
        message: String,
        /// The locked resource
        resource: Option<String>,
    },

    /// Rate Limited (429) - Too many requests
    #[error("Rate Limited (429): {message}")]
    RateLimited {
        message: String,
        /// Seconds until retry is allowed
        retry_after: Option<u64>,
    },

    /// Internal Server Error (500)
    #[error("Internal Server Error (500): {message}")]
    InternalServerError {
        message: String,
        /// Request ID for support purposes
        request_id: Option<String>,
    },

    /// Service Unavailable (503)
    #[error("Service Unavailable (503): {message}")]
    ServiceUnavailable {
        message: String,
        /// Seconds until service might be available
        retry_after: Option<u64>,
    },

    /// Generic API error with status code
    #[error("API error ({code}): {message}")]
    ApiError {
        code: u16,
        message: String,
        /// The endpoint that returned the error
        endpoint: Option<String>,
    },

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// I/O error (file operations)
    #[error("I/O error: {0}")]
    IoError(String),

    /// URL parsing error
    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

impl FilesError {
    /// Create a NotFound error with context
    pub fn not_found(message: impl Into<String>) -> Self {
        FilesError::NotFound {
            message: message.into(),
            resource_type: None,
            path: None,
        }
    }

    /// Create a NotFound error for a specific resource type
    pub fn not_found_resource(
        message: impl Into<String>,
        resource_type: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        FilesError::NotFound {
            message: message.into(),
            resource_type: Some(resource_type.into()),
            path: Some(path.into()),
        }
    }

    /// Create a BadRequest error with optional field context
    pub fn bad_request(message: impl Into<String>) -> Self {
        FilesError::BadRequest {
            message: message.into(),
            field: None,
        }
    }

    /// Create a BadRequest error with field context
    pub fn bad_request_field(message: impl Into<String>, field: impl Into<String>) -> Self {
        FilesError::BadRequest {
            message: message.into(),
            field: Some(field.into()),
        }
    }

    /// Create an UnprocessableEntity error with validation context
    pub fn validation_failed(
        message: impl Into<String>,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        FilesError::UnprocessableEntity {
            message: message.into(),
            field: Some(field.into()),
            value: Some(value.into()),
        }
    }

    /// Create a RateLimited error with retry-after context
    pub fn rate_limited(message: impl Into<String>, retry_after: Option<u64>) -> Self {
        FilesError::RateLimited {
            message: message.into(),
            retry_after,
        }
    }

    /// Add resource context to a NotFound error
    pub fn with_resource_type(mut self, resource_type: impl Into<String>) -> Self {
        if let FilesError::NotFound {
            resource_type: rt, ..
        } = &mut self
        {
            *rt = Some(resource_type.into());
        }
        self
    }

    /// Add path context to a NotFound error
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        if let FilesError::NotFound { path: p, .. } = &mut self {
            *p = Some(path.into());
        }
        self
    }

    /// Add field context to a BadRequest error
    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        if let FilesError::BadRequest { field: f, .. } = &mut self {
            *f = Some(field.into());
        }
        self
    }

    /// Extract the HTTP status code if this is an API error
    pub fn status_code(&self) -> Option<u16> {
        match self {
            FilesError::BadRequest { .. } => Some(400),
            FilesError::AuthenticationFailed { .. } => Some(401),
            FilesError::Forbidden { .. } => Some(403),
            FilesError::NotFound { .. } => Some(404),
            FilesError::Conflict { .. } => Some(409),
            FilesError::PreconditionFailed { .. } => Some(412),
            FilesError::UnprocessableEntity { .. } => Some(422),
            FilesError::Locked { .. } => Some(423),
            FilesError::RateLimited { .. } => Some(429),
            FilesError::InternalServerError { .. } => Some(500),
            FilesError::ServiceUnavailable { .. } => Some(503),
            FilesError::ApiError { code, .. } => Some(*code),
            _ => None,
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            FilesError::RateLimited { .. }
                | FilesError::ServiceUnavailable { .. }
                | FilesError::InternalServerError { .. }
        )
    }

    /// Get retry-after duration if available
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            FilesError::RateLimited { retry_after, .. } => *retry_after,
            FilesError::ServiceUnavailable { retry_after, .. } => *retry_after,
            _ => None,
        }
    }
}

/// Result type for Files.com operations
pub type Result<T> = std::result::Result<T, FilesError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_with_context() {
        let error = FilesError::not_found_resource("File not found", "file", "/path/to/file.txt");
        assert!(matches!(error, FilesError::NotFound { .. }));
        assert!(error.to_string().contains("Not Found"));
    }

    #[test]
    fn test_bad_request_with_field() {
        let error = FilesError::bad_request_field("Invalid value", "username");
        if let FilesError::BadRequest { field, .. } = error {
            assert_eq!(field, Some("username".to_string()));
        } else {
            panic!("Expected BadRequest error");
        }
    }

    #[test]
    fn test_validation_failed() {
        let error = FilesError::validation_failed("Invalid email format", "email", "not-an-email");
        if let FilesError::UnprocessableEntity { field, value, .. } = error {
            assert_eq!(field, Some("email".to_string()));
            assert_eq!(value, Some("not-an-email".to_string()));
        } else {
            panic!("Expected UnprocessableEntity error");
        }
    }

    #[test]
    fn test_rate_limited_with_retry() {
        let error = FilesError::rate_limited("Too many requests", Some(60));
        assert_eq!(error.retry_after(), Some(60));
        assert!(error.is_retryable());
    }

    #[test]
    fn test_status_code_extraction() {
        assert_eq!(FilesError::not_found("test").status_code(), Some(404));
        assert_eq!(FilesError::bad_request("test").status_code(), Some(400));
        assert_eq!(
            FilesError::rate_limited("test", None).status_code(),
            Some(429)
        );
    }

    #[test]
    fn test_is_retryable() {
        assert!(FilesError::rate_limited("test", None).is_retryable());
        assert!(
            FilesError::InternalServerError {
                message: "test".to_string(),
                request_id: None
            }
            .is_retryable()
        );
        assert!(!FilesError::not_found("test").is_retryable());
    }

    #[test]
    fn test_builder_pattern() {
        let error = FilesError::not_found("File not found")
            .with_resource_type("file")
            .with_path("/test.txt");

        if let FilesError::NotFound {
            resource_type,
            path,
            ..
        } = error
        {
            assert_eq!(resource_type, Some("file".to_string()));
            assert_eq!(path, Some("/test.txt".to_string()));
        } else {
            panic!("Expected NotFound error");
        }
    }
}
