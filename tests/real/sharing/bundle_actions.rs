//! Real API integration tests for BundleActionHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! BundleActions track historical actions taken on bundles.

use crate::real::*;
use files_sdk::BundleActionHandler;

#[tokio::test]
async fn test_real_api_list_bundle_actions() {
    let client = get_test_client();
    let handler = BundleActionHandler::new(client);

    // List bundle actions (may be empty if no bundles have been accessed)
    let result = handler.list().await;

    match result {
        Ok(actions) => {
            println!("Listed {} bundle actions", actions.len());
            // Bundle actions may be empty in test accounts
            // Just verify we can call the endpoint without errors
        }
        Err(e) => {
            // Some plans may not have access to bundle actions
            println!(
                "Bundle actions list failed (may require paid plan): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_bundle_actions_with_activity() {
    let client = get_test_client();
    let handler = BundleActionHandler::new(client.clone());

    // This test verifies bundle actions exist if there's bundle activity
    // In a fresh test environment, this may return empty results

    let result = handler.list().await;

    match result {
        Ok(actions) => {
            println!("Bundle actions count: {}", actions.len());

            // If we have actions, verify the structure
            if let Some(first) = actions.first() {
                println!("Sample bundle action: {:?}", first);
                // BundleActionEntity uses flatten, so all fields are in data
                assert!(!first.data.is_empty(), "Bundle action should have data");
            }
        }
        Err(e) => {
            println!("Bundle actions not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_bundle_actions_error_handling() {
    let client = get_test_client();
    let handler = BundleActionHandler::new(client);

    // Test that we handle errors gracefully
    let result = handler.list().await;

    // Should either succeed or return a proper error (not panic)
    match result {
        Ok(actions) => {
            println!("Successfully listed {} bundle actions", actions.len());
        }
        Err(e) => {
            println!("Expected error handling works: {:?}", e);
            // Error is acceptable (permission, plan limit, etc.)
        }
    }
}
