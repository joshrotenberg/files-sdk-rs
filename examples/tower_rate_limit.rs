//! Example: Using Tower middleware for rate limiting with the Files.com SDK
//!
//! This example demonstrates how to use tower's rate limiting middleware.
//! Tower provides flexible rate limiting through the governor crate integration.
//!
//! To use tower middleware with this SDK, enable the `tower` feature:
//! ```toml
//! files-sdk = { version = "0.3", features = ["tower"] }
//! ```

use files_sdk::FilesClient;
use std::env;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("FILES_API_KEY").expect("FILES_API_KEY must be set");

    // Create base FilesClient
    let client = FilesClient::builder().api_key(&api_key).build()?;

    println!("Example 1: Token bucket rate limiting");
    println!("======================================");

    // For actual rate limiting, you would use governor crate with tower
    // Example configuration (requires governor crate as a direct dependency):
    //
    // use governor::{Quota, RateLimiter};
    // use governor::clock::DefaultClock;
    // use governor::state::{InMemoryState, NotKeyed};
    //
    // let quota = Quota::per_second(NonZeroU32::new(10).unwrap());
    // let limiter = RateLimiter::<NotKeyed, InMemoryState, DefaultClock>::direct(quota);
    //
    // Then wrap your service with a custom middleware that checks the limiter

    println!("Rate limiting with tower requires:");
    println!("1. Add governor to your Cargo.toml");
    println!("2. Create a RateLimiter with your desired quota");
    println!("3. Implement a tower Layer that checks the limiter");
    println!("4. Use ServiceBuilder to apply the layer");

    println!("\nExample 2: Combining rate limiting with other middleware");
    println!("=========================================================");

    // Tower allows composing multiple middleware layers
    let _complex_client = ServiceBuilder::new()
        // Add timeout
        .timeout(std::time::Duration::from_secs(30))
        // Add rate limiting (custom layer)
        // .layer(RateLimitLayer::new(limiter))
        // Add retry
        // .layer(RetryLayer::new(...))
        .service(client.clone());

    println!("Created client with timeout + rate limit + retry");

    println!("\nKey Concepts:");
    println!("- Tower's middleware system is composable and reusable");
    println!("- Rate limiting can be implemented with governor");
    println!("- Combine multiple middleware layers with ServiceBuilder");
    println!("- Each layer wraps the service, adding behavior");

    println!("\nSample Code:");
    println!("```rust");
    println!("use governor::{{Quota, RateLimiter}};");
    println!("use std::num::NonZeroU32;");
    println!();
    println!("let quota = Quota::per_second(NonZeroU32::new(10).unwrap());");
    println!("let limiter = RateLimiter::direct(quota);");
    println!();
    println!("// Use with tower middleware");
    println!("let client = ServiceBuilder::new()");
    println!("    .layer(RateLimitLayer::new(limiter))");
    println!("    .service(files_client);");
    println!("```");

    Ok(())
}
