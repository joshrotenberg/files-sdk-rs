//! Files.com API client core implementation
//!
//! This module contains the core HTTP client for interacting with the Files.com REST API.
//! It provides authentication handling, request/response processing, and error management.
//!
//! The client is designed around a builder pattern for flexible configuration and supports
//! both typed and untyped API interactions.

use crate::{FilesError, Result};
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

#[cfg(feature = "tracing")]
use tracing::{debug, error, instrument, warn};

/// Builder for constructing a FilesClient with custom configuration
///
/// Provides a fluent interface for configuring API credentials, base URL, timeouts,
/// and other client settings before creating the final FilesClient instance.
///
/// # Examples
///
/// ```rust,no_run
/// use files_sdk::FilesClient;
///
/// // Basic configuration
/// let client = FilesClient::builder()
///     .api_key("your-api-key")
///     .build()?;
///
/// // Advanced configuration
/// let client = FilesClient::builder()
///     .api_key("your-api-key")
///     .base_url("https://app.files.com/api/rest/v1".to_string())
///     .timeout(std::time::Duration::from_secs(120))
///     .build()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone)]
pub struct FilesClientBuilder {
    api_key: Option<String>,
    base_url: String,
    timeout: Duration,
}

impl Default for FilesClientBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: "https://app.files.com/api/rest/v1".to_string(),
            timeout: Duration::from_secs(60),
        }
    }
}

impl FilesClientBuilder {
    /// Sets the API key for authentication
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Files.com API key
    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets a custom base URL for the API
    ///
    /// # Arguments
    ///
    /// * `base_url` - Custom base URL (useful for testing or regional endpoints)
    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Sets the request timeout duration
    ///
    /// # Arguments
    ///
    /// * `timeout` - Maximum duration for API requests
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Builds the FilesClient instance
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - API key is not set
    /// - HTTP client cannot be constructed
    pub fn build(self) -> Result<FilesClient> {
        let api_key = self
            .api_key
            .ok_or_else(|| FilesError::ConfigError("API key is required".to_string()))?;

        let client = Client::builder()
            .timeout(self.timeout)
            .build()
            .map_err(|e| FilesError::ConfigError(format!("Failed to build HTTP client: {}", e)))?;

        Ok(FilesClient {
            inner: Arc::new(FilesClientInner {
                api_key,
                base_url: self.base_url,
                client,
            }),
        })
    }
}

/// Internal client state
#[derive(Debug)]
pub(crate) struct FilesClientInner {
    pub(crate) api_key: String,
    pub(crate) base_url: String,
    pub(crate) client: Client,
}

/// Files.com API client
///
/// The main client for interacting with the Files.com API. Handles authentication,
/// request construction, and response processing.
///
/// # Examples
///
/// ```rust,no_run
/// use files_sdk::FilesClient;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = FilesClient::builder()
///     .api_key("your-api-key")
///     .build()?;
///
/// // Use with handlers
/// let file_handler = files_sdk::FileHandler::new(client.clone());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct FilesClient {
    pub(crate) inner: Arc<FilesClientInner>,
}

impl FilesClient {
    /// Creates a new FilesClientBuilder
    pub fn builder() -> FilesClientBuilder {
        FilesClientBuilder::default()
    }

    /// Performs a GET request to the Files.com API
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path (without base URL)
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or returns a non-success status code
    #[cfg_attr(feature = "tracing", instrument(skip(self), fields(method = "GET")))]
    pub async fn get_raw(&self, path: &str) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.inner.base_url, path);

        #[cfg(feature = "tracing")]
        debug!("Making GET request to {}", path);

        let response = self
            .inner
            .client
            .get(&url)
            .header("X-FilesAPI-Key", &self.inner.api_key)
            .send()
            .await?;

        #[cfg(feature = "tracing")]
        debug!("GET response status: {}", response.status());

        self.handle_response(response).await
    }

    /// Performs a POST request to the Files.com API
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path (without base URL)
    /// * `body` - Request body (will be serialized to JSON)
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or returns a non-success status code
    #[cfg_attr(
        feature = "tracing",
        instrument(skip(self, body), fields(method = "POST"))
    )]
    pub async fn post_raw<T: Serialize>(&self, path: &str, body: T) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.inner.base_url, path);

        #[cfg(feature = "tracing")]
        debug!("Making POST request to {}", path);

        let response = self
            .inner
            .client
            .post(&url)
            .header("X-FilesAPI-Key", &self.inner.api_key)
            .json(&body)
            .send()
            .await?;

        #[cfg(feature = "tracing")]
        debug!("POST response status: {}", response.status());

        self.handle_response(response).await
    }

    /// Performs a PATCH request to the Files.com API
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path (without base URL)
    /// * `body` - Request body (will be serialized to JSON)
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or returns a non-success status code
    #[cfg_attr(
        feature = "tracing",
        instrument(skip(self, body), fields(method = "PATCH"))
    )]
    pub async fn patch_raw<T: Serialize>(&self, path: &str, body: T) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.inner.base_url, path);

        #[cfg(feature = "tracing")]
        debug!("Making PATCH request to {}", path);

        let response = self
            .inner
            .client
            .patch(&url)
            .header("X-FilesAPI-Key", &self.inner.api_key)
            .json(&body)
            .send()
            .await?;

        #[cfg(feature = "tracing")]
        debug!("PATCH response status: {}", response.status());

        self.handle_response(response).await
    }

    /// Performs a DELETE request to the Files.com API
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path (without base URL)
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or returns a non-success status code
    #[cfg_attr(feature = "tracing", instrument(skip(self), fields(method = "DELETE")))]
    pub async fn delete_raw(&self, path: &str) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.inner.base_url, path);

        #[cfg(feature = "tracing")]
        debug!("Making DELETE request to {}", path);

        let response = self
            .inner
            .client
            .delete(&url)
            .header("X-FilesAPI-Key", &self.inner.api_key)
            .send()
            .await?;

        #[cfg(feature = "tracing")]
        debug!("DELETE response status: {}", response.status());

        self.handle_response(response).await
    }

    /// Performs a POST request with form data to the Files.com API
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path (without base URL)
    /// * `form` - Form data as key-value pairs
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or returns a non-success status code
    pub async fn post_form<T: Serialize>(&self, path: &str, form: T) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.inner.base_url, path);

        let response = self
            .inner
            .client
            .post(&url)
            .header("X-FilesAPI-Key", &self.inner.api_key)
            .form(&form)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Handles HTTP response and converts to Result
    ///
    /// Processes status codes and extracts error information when applicable
    async fn handle_response(&self, response: reqwest::Response) -> Result<serde_json::Value> {
        let status = response.status();

        if status.is_success() {
            // Handle 204 No Content
            if status.as_u16() == 204 {
                #[cfg(feature = "tracing")]
                debug!("Received 204 No Content response");
                return Ok(serde_json::Value::Null);
            }

            let value = response.json().await?;
            Ok(value)
        } else {
            let status_code = status.as_u16();
            let error_body = response.text().await.unwrap_or_default();

            #[cfg(feature = "tracing")]
            warn!(
                status_code = status_code,
                error_body = %error_body,
                "API request failed"
            );

            // Try to parse error message from JSON
            let message = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&error_body) {
                json.get("error")
                    .or_else(|| json.get("message"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(&error_body)
                    .to_string()
            } else {
                error_body
            };

            let error = match status_code {
                400 => FilesError::BadRequest {
                    message,
                    field: None,
                },
                401 => FilesError::AuthenticationFailed {
                    message,
                    auth_type: None,
                },
                403 => FilesError::Forbidden {
                    message,
                    resource: None,
                },
                404 => FilesError::NotFound {
                    message,
                    resource_type: None,
                    path: None,
                },
                409 => FilesError::Conflict {
                    message,
                    resource: None,
                },
                412 => FilesError::PreconditionFailed {
                    message,
                    condition: None,
                },
                422 => FilesError::UnprocessableEntity {
                    message,
                    field: None,
                    value: None,
                },
                423 => FilesError::Locked {
                    message,
                    resource: None,
                },
                429 => FilesError::RateLimited {
                    message,
                    retry_after: None, // TODO: Parse Retry-After header
                },
                500 => FilesError::InternalServerError {
                    message,
                    request_id: None, // TODO: Parse request ID from headers
                },
                503 => FilesError::ServiceUnavailable {
                    message,
                    retry_after: None, // TODO: Parse Retry-After header
                },
                _ => FilesError::ApiError {
                    code: status_code,
                    message,
                    endpoint: None,
                },
            };

            #[cfg(feature = "tracing")]
            error!(error = ?error, "Returning error to caller");

            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default() {
        let builder = FilesClientBuilder::default();
        assert_eq!(
            builder.base_url,
            "https://app.files.com/api/rest/v1".to_string()
        );
        assert_eq!(builder.timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_builder_custom() {
        let builder = FilesClientBuilder::default()
            .api_key("test-key")
            .base_url("https://custom.example.com")
            .timeout(Duration::from_secs(120));

        assert_eq!(builder.api_key, Some("test-key".to_string()));
        assert_eq!(builder.base_url, "https://custom.example.com");
        assert_eq!(builder.timeout, Duration::from_secs(120));
    }

    #[test]
    fn test_builder_missing_api_key() {
        let result = FilesClientBuilder::default().build();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FilesError::ConfigError(_)));
    }

    #[test]
    fn test_builder_success() {
        let result = FilesClientBuilder::default().api_key("test-key").build();
        assert!(result.is_ok());
    }
}
