//! Request/Response interceptors for observability and debugging
//!
//! This module provides middleware for intercepting HTTP requests and responses,
//! useful for logging, metrics, tracing, and debugging.
//!
//! # Examples
//!
//! ```rust,no_run
//! use files_sdk::{FilesClient, interceptors::LoggingInterceptor};
//! use reqwest_middleware::ClientBuilder;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = FilesClient::builder()
//!     .api_key("your-api-key")
//!     .with_interceptor(LoggingInterceptor::new())
//!     .build()?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use std::time::Instant;

/// Logging interceptor that prints request/response information
///
/// Logs:
/// - Request method and URL
/// - Response status code
/// - Request duration
pub struct LoggingInterceptor {
    /// Include request/response headers in logs
    pub include_headers: bool,
}

impl LoggingInterceptor {
    /// Create a new logging interceptor
    pub fn new() -> Self {
        Self {
            include_headers: false,
        }
    }

    /// Create a logging interceptor that includes headers
    pub fn with_headers() -> Self {
        Self {
            include_headers: true,
        }
    }
}

impl Default for LoggingInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Middleware for LoggingInterceptor {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let method = req.method().clone();
        let url = req.url().clone();
        let start = Instant::now();

        println!("[files-sdk] {} {}", method, url);

        if self.include_headers {
            println!("[files-sdk] Request headers:");
            for (name, value) in req.headers() {
                if name == "x-filesapi-key" {
                    println!("  {}: [REDACTED]", name);
                } else if let Ok(val) = value.to_str() {
                    println!("  {}: {}", name, val);
                }
            }
        }

        let res = next.run(req, extensions).await;

        let duration = start.elapsed();

        match &res {
            Ok(response) => {
                println!(
                    "[files-sdk] {} {} - {} ({:?})",
                    method,
                    url,
                    response.status(),
                    duration
                );

                if self.include_headers {
                    println!("[files-sdk] Response headers:");
                    for (name, value) in response.headers() {
                        if let Ok(val) = value.to_str() {
                            println!("  {}: {}", name, val);
                        }
                    }
                }
            }
            Err(e) => {
                println!(
                    "[files-sdk] {} {} - ERROR: {} ({:?})",
                    method, url, e, duration
                );
            }
        }

        res
    }
}

/// Metrics interceptor for collecting request statistics
///
/// Tracks:
/// - Total requests
/// - Request duration
/// - Status codes
/// - Errors
pub struct MetricsInterceptor {
    /// Callback function for metrics
    callback: Box<dyn Fn(RequestMetrics) + Send + Sync>,
}

/// Metrics collected for each request
#[derive(Debug, Clone)]
pub struct RequestMetrics {
    /// HTTP method
    pub method: String,
    /// Request URL
    pub url: String,
    /// Response status code (if successful)
    pub status_code: Option<u16>,
    /// Request duration
    pub duration: std::time::Duration,
    /// Whether the request succeeded
    pub success: bool,
    /// Error message (if failed)
    pub error: Option<String>,
}

impl MetricsInterceptor {
    /// Create a new metrics interceptor with a callback
    ///
    /// # Examples
    ///
    /// ```rust
    /// use files_sdk::interceptors::MetricsInterceptor;
    ///
    /// let interceptor = MetricsInterceptor::new(|metrics| {
    ///     println!("Request to {} took {:?}", metrics.url, metrics.duration);
    /// });
    /// ```
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(RequestMetrics) + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(callback),
        }
    }
}

#[async_trait]
impl Middleware for MetricsInterceptor {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let method = req.method().to_string();
        let url = req.url().to_string();
        let start = Instant::now();

        let res = next.run(req, extensions).await;
        let duration = start.elapsed();

        let metrics = match &res {
            Ok(response) => RequestMetrics {
                method,
                url,
                status_code: Some(response.status().as_u16()),
                duration,
                success: response.status().is_success(),
                error: None,
            },
            Err(e) => RequestMetrics {
                method,
                url,
                status_code: None,
                duration,
                success: false,
                error: Some(e.to_string()),
            },
        };

        (self.callback)(metrics);

        res
    }
}

/// Custom interceptor using closures for request/response handling
///
/// Allows inline interceptor logic without creating a struct.
pub struct FnInterceptor {
    handler: Box<dyn Fn(&Request) -> Option<Box<dyn Fn(&Result<Response>) + Send>> + Send + Sync>,
}

impl FnInterceptor {
    /// Create a new function-based interceptor
    ///
    /// # Examples
    ///
    /// ```rust
    /// use files_sdk::interceptors::FnInterceptor;
    ///
    /// let interceptor = FnInterceptor::new(|req| {
    ///     println!("Before: {}", req.url());
    ///     Some(Box::new(|res| {
    ///         println!("After: {:?}", res);
    ///     }))
    /// });
    /// ```
    pub fn new<F, G>(handler: F) -> Self
    where
        F: Fn(&Request) -> Option<G> + Send + Sync + 'static,
        G: Fn(&Result<Response>) + Send + 'static,
    {
        Self {
            handler: Box::new(move |req| {
                handler(req).map(|g| Box::new(g) as Box<dyn Fn(&Result<Response>) + Send>)
            }),
        }
    }
}

#[async_trait]
impl Middleware for FnInterceptor {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let callback = (self.handler)(&req);

        let res = next.run(req, extensions).await;

        if let Some(cb) = callback {
            cb(&res);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_interceptor_creation() {
        let _interceptor = LoggingInterceptor::new();
        let _interceptor_with_headers = LoggingInterceptor::with_headers();
    }

    #[test]
    fn test_metrics_interceptor_creation() {
        let _interceptor = MetricsInterceptor::new(|_metrics| {
            // No-op for test
        });
    }

    #[test]
    fn test_fn_interceptor_creation() {
        let _interceptor = FnInterceptor::new(|_req| {
            Some(|_res: &Result<Response>| {
                // No-op for test
            })
        });
    }
}
