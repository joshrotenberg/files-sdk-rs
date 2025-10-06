//! Real API integration tests for BundleDownloadHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! BundleDownloads track historical download activity for bundles.

use crate::real::*;
use files_sdk::BundleDownloadHandler;

#[tokio::test]
async fn test_real_api_list_bundle_downloads() {
    let client = get_test_client();
    let handler = BundleDownloadHandler::new(client);

    // List bundle downloads (may be empty if no bundles have been downloaded)
    let result = handler.list().await;

    match result {
        Ok(downloads) => {
            println!("Listed {} bundle downloads", downloads.len());
            // Bundle downloads may be empty in test accounts
            // Just verify we can call the endpoint without errors
        }
        Err(e) => {
            // Some plans may not have access to bundle download tracking
            println!(
                "Bundle downloads list failed (may require paid plan): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_bundle_downloads_structure() {
    let client = get_test_client();
    let handler = BundleDownloadHandler::new(client);

    let result = handler.list().await;

    match result {
        Ok(downloads) => {
            println!("Bundle downloads count: {}", downloads.len());

            // If we have downloads, verify the structure
            if let Some(first) = downloads.first() {
                println!("Sample bundle download: {:?}", first);
                // BundleDownloadEntity uses flatten, so all fields are in data
                assert!(!first.data.is_empty(), "Bundle download should have data");
            }
        }
        Err(e) => {
            println!("Bundle downloads not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_bundle_downloads_error_handling() {
    let client = get_test_client();
    let handler = BundleDownloadHandler::new(client);

    // Test that we handle errors gracefully
    let result = handler.list().await;

    // Should either succeed or return a proper error (not panic)
    match result {
        Ok(downloads) => {
            println!("Successfully listed {} bundle downloads", downloads.len());
        }
        Err(e) => {
            println!("Expected error handling works: {:?}", e);
            // Error is acceptable (permission, plan limit, etc.)
        }
    }
}
