//! Example: Using Tower middleware for retry logic with the Files.com SDK
//!
//! This example demonstrates how to use tower middleware for retry logic.
//! Tower provides a flexible, composable middleware system.
//!
//! To use tower middleware with this SDK, enable the `tower` feature:
//! ```toml
//! files-sdk = { version = "0.3", features = ["tower"] }
//! ```
//!
//! For retry functionality, you'll also need:
//! ```toml
//! tower = "0.5"
//! tower-http = { version = "0.6", features = ["retry"] }
//! ```

use files_sdk::FilesClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("FILES_API_KEY").expect("FILES_API_KEY must be set");

    // Create base FilesClient
    let _client = FilesClient::builder().api_key(&api_key).build()?;

    println!("Tower Retry Example");
    println!("===================\n");

    println!("To add retry logic with tower, add these dependencies:");
    println!("  tower = \"0.5\"");
    println!("  tower-http = {{ version = \"0.6\", features = [\"retry\"] }}");
    println!();

    println!("Then use RetryLayer:");
    println!("```rust");
    println!("use tower::ServiceBuilder;");
    println!("use tower_http::retry::RetryLayer;");
    println!("use tower_http::classify::ServerErrorsAsFailures;");
    println!();
    println!("let client = FilesClient::builder().api_key(\"key\").build()?;");
    println!();
    println!("let retrying_client = ServiceBuilder::new()");
    println!("    .layer(RetryLayer::new(ServerErrorsAsFailures::default()))");
    println!("    .service(client);");
    println!("```");
    println!();

    println!("Key Features:");
    println!("- Automatic retry on 5xx server errors");
    println!("- Exponential backoff (configurable)");
    println!("- Custom retry policies via RetryPolicy trait");
    println!("- Composable with other middleware layers");

    Ok(())
}
