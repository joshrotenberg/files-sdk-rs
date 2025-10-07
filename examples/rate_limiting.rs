//! Rate Limiting Example
//!
//! Demonstrates client-side rate limiting to prevent hitting Files.com API limits.
//!
//! Run with:
//! ```bash
//! FILES_API_KEY=your-key cargo run --example rate_limiting
//! ```

use files_sdk::files::FolderHandler;
use files_sdk::{FilesClient, Result};
use std::env;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = env::var("FILES_API_KEY").expect("FILES_API_KEY environment variable required");

    println!("Files.com Rate Limiting Examples\n");
    println!("=================================\n");

    // Example 1: No rate limiting (default)
    println!("1. No Rate Limiting (Unlimited):");
    println!("   - All requests sent immediately");
    println!("   - Risk of hitting API rate limits\n");

    let client_unlimited = FilesClient::builder().api_key(&api_key).build()?;

    let start = Instant::now();
    make_requests(client_unlimited, 5).await?;
    let elapsed = start.elapsed();

    println!("   Completed 5 requests in {:?}", elapsed);
    println!("   Average: {:?} per request\n", elapsed / 5);

    // Example 2: Rate limited to 2 requests per second
    println!("2. Rate Limited (2 requests/second):");
    println!("   - Token bucket algorithm");
    println!("   - Automatic throttling when limit reached");
    println!("   - Prevents 429 errors\n");

    let client_limited = FilesClient::builder()
        .api_key(&api_key)
        .rate_limit(2) // 2 requests per second
        .build()?;

    let start = Instant::now();
    make_requests(client_limited, 5).await?;
    let elapsed = start.elapsed();

    println!("   Completed 5 requests in {:?}", elapsed);
    println!("   Average: {:?} per request\n", elapsed / 5);
    println!("   Note: Slower due to rate limiting (expected)\n");

    // Example 3: Production configuration
    println!("3. Production Configuration:");
    println!("   - Rate limit: 10 requests/second");
    println!("   - Retries: 3 attempts");
    println!("   - Timeout: 60s");
    println!("   - Balanced for reliability and performance\n");

    let client_production = FilesClient::builder()
        .api_key(&api_key)
        .rate_limit(10)
        .max_retries(3)
        .timeout(Duration::from_secs(60))
        .build()?;

    let folder_handler = FolderHandler::new(client_production);
    match folder_handler.list_folder("/", None, None).await {
        Ok(_) => println!("   ✓ Successfully listed root directory with production config"),
        Err(e) => println!("   ✗ Failed: {}", e),
    }

    println!("\nRate Limiting Behavior:");
    println!("=======================");
    println!("- Token bucket refills at configured rate (requests/second)");
    println!("- Requests block if no tokens available");
    println!("- Allows short bursts up to bucket capacity");
    println!("- Prevents wasteful retries from 429 errors");
    println!("- Recommended for bulk operations or high-frequency access");

    Ok(())
}

/// Helper function to make multiple requests
async fn make_requests(client: FilesClient, count: usize) -> Result<()> {
    let folder_handler = FolderHandler::new(client);

    for i in 1..=count {
        let start = Instant::now();
        folder_handler
            .list_folder("/", None, Some("1".to_string()))
            .await?;
        println!(
            "   Request {}/{} completed in {:?}",
            i,
            count,
            start.elapsed()
        );
    }

    Ok(())
}
