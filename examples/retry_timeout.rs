//! Example demonstrating retry logic and timeout configuration
//!
//! This example shows how to configure automatic retries with exponential backoff
//! and various timeout strategies for the Files.com SDK.
//!
//! Run with:
//! ```bash
//! FILES_API_KEY=your-key cargo run --example retry_timeout
//! ```

use files_sdk::files::FolderHandler;
use files_sdk::{FilesClient, Result};
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from environment
    let api_key = env::var("FILES_API_KEY").expect("FILES_API_KEY must be set");

    println!("=== Files.com SDK - Retry and Timeout Examples ===\n");

    // Example 1: Default configuration
    println!("1. Default Configuration:");
    println!("   - Per-request timeout: 60s");
    println!("   - Max retries: 3");
    println!("   - Base retry delay: 1s");
    println!("   - Retries on: 429, 500, 502, 503, 504\n");

    let _client_default = FilesClient::builder().api_key(&api_key).build()?;

    // Example 2: Custom retry configuration
    println!("2. Aggressive Retry Configuration:");
    println!("   - Max retries: 5");
    println!("   - Base delay: 2s (exponential backoff: 2s → 4s → 8s...)\n");

    let _client_aggressive = FilesClient::builder()
        .api_key(&api_key)
        .max_retries(5)
        .retry_base_delay(Duration::from_secs(2))
        .build()?;

    // Example 3: No retries (fail fast)
    println!("3. No Retries (Fail Fast):");
    println!("   - Max retries: 0");
    println!("   - Per-request timeout: 30s\n");

    let _client_no_retry = FilesClient::builder()
        .api_key(&api_key)
        .max_retries(0)
        .timeout(Duration::from_secs(30))
        .build()?;

    // Example 4: Long timeout for large uploads
    println!("4. Long Timeout for Large Operations:");
    println!("   - Per-request timeout: 300s (5 minutes)");
    println!("   - Max retries: 2\n");

    let _client_long_timeout = FilesClient::builder()
        .api_key(&api_key)
        .timeout(Duration::from_secs(300))
        .max_retries(2)
        .build()?;

    // Example 5: Production-recommended configuration
    println!("5. Production-Recommended Configuration:");
    println!("   - Per-request timeout: 60s");
    println!("   - Max retries: 3");
    println!("   - Base delay: 1s");
    println!("   - Exponential backoff with jitter\n");

    let client_production = FilesClient::builder()
        .api_key(&api_key)
        .max_retries(3)
        .retry_base_delay(Duration::from_secs(1))
        .timeout(Duration::from_secs(60))
        .build()?;

    // Demonstrate actual usage
    println!("=== Testing Retry Logic ===\n");

    let folder_handler = FolderHandler::new(client_production);

    match folder_handler.list_folder("/", None, None).await {
        Ok(_) => {
            println!("✓ Successfully listed root directory");
            println!("  (If there were transient errors, they were automatically retried)");
        }
        Err(e) => {
            println!("✗ Failed after retries: {}", e);
            println!("  (The SDK automatically retried transient errors before giving up)");
        }
    }

    println!("\n=== Timeout Behavior Explained ===\n");
    println!("Per-Request Timeout:");
    println!("  - Each retry attempt has the configured timeout");
    println!("  - Example: 3 retries × 60s = up to 180s total");
    println!("  - Each attempt starts fresh with full timeout");
    println!();
    println!("Retry Strategy:");
    println!("  - Automatic for transient errors only");
    println!("  - Exponential backoff prevents overwhelming servers");
    println!("  - Jitter prevents thundering herd");
    println!();
    println!("When Retries Happen:");
    println!("  - ✓ 429 Too Many Requests (rate limit)");
    println!("  - ✓ 500 Internal Server Error");
    println!("  - ✓ 502 Bad Gateway");
    println!("  - ✓ 503 Service Unavailable");
    println!("  - ✓ 504 Gateway Timeout");
    println!();
    println!("When Retries DON'T Happen:");
    println!("  - ✗ 400 Bad Request (client error)");
    println!("  - ✗ 401 Unauthorized (auth error)");
    println!("  - ✗ 404 Not Found (missing resource)");
    println!("  - ✗ Network errors (connection failed)");

    Ok(())
}
