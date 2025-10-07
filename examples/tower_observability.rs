//! Example: Using Tower middleware for observability with the Files.com SDK
//!
//! This example demonstrates how to use tower-http's tracing middleware
//! to add observability to your Files.com API calls.
//!
//! To use tower middleware with this SDK, enable both features:
//! ```toml
//! files-sdk = { version = "0.3", features = ["tower", "tracing"] }
//! tower-http = { version = "0.6", features = ["trace"] }
//! ```

use files_sdk::FilesClient;
use std::env;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber to see the logs
    tracing_subscriber::fmt()
        .with_env_filter("tower_http=debug,files_sdk=debug")
        .init();

    let api_key = env::var("FILES_API_KEY").expect("FILES_API_KEY must be set");

    // Create base FilesClient
    let _client = FilesClient::builder().api_key(&api_key).build()?;

    println!("Example 1: Request/Response tracing");
    println!("====================================");

    // Add TraceLayer for automatic request/response logging
    let _traced_client = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .service(_client.clone());

    println!("Client wrapped with TraceLayer - all requests will be logged");

    println!("\nExample 2: Custom trace configuration");
    println!("======================================");

    // You can customize what gets logged
    let _custom_traced_client = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO),
                )
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::DEBUG))
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                ),
        )
        .service(_client.clone());

    println!("Client with custom trace levels configured");

    println!("\nExample 3: Full observability stack");
    println!("====================================");

    let _production_client = ServiceBuilder::new()
        // Timeout for all requests
        .timeout(std::time::Duration::from_secs(30))
        // Trace all requests
        .layer(TraceLayer::new_for_http())
        .service(_client);

    println!("Production-ready client with:");
    println!("- Timeouts");
    println!("- Request/response tracing");

    println!("\nKey Concepts:");
    println!("- TraceLayer provides automatic request/response logging");
    println!("- Customize span creation and log levels");
    println!("- Use ServiceBuilder to compose multiple layers");

    println!("\nSample Output (when making requests):");
    println!("  DEBUG tower_http: request");
    println!("  DEBUG tower_http: response status=200 latency=123ms");

    Ok(())
}
