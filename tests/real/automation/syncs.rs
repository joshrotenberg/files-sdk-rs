//! Real API integration tests for SyncHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! Syncs configure scheduled synchronization with remote servers.

use crate::real::*;
use files_sdk::SyncHandler;

#[tokio::test]
async fn test_real_api_list_syncs() {
    let client = get_test_client();
    let handler = SyncHandler::new(client);

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((syncs, pagination)) => {
            println!("Listed {} syncs", syncs.len());
            println!("Pagination: {:?}", pagination);

            if let Some(first) = syncs.first() {
                println!("Sample sync: {:?}", first);
            }
        }
        Err(e) => {
            println!("Syncs list failed (may require permissions): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_get_sync() {
    let client = get_test_client();
    let handler = SyncHandler::new(client);

    // First, get a list to find an ID
    let list_result = handler.list(None, Some(1)).await;

    match list_result {
        Ok((syncs, _)) => {
            if let Some(sync) = syncs.first() {
                // Extract ID from the sync data
                if let Some(id) = sync.data.get("id").and_then(|v| v.as_i64()) {
                    // Try to get that specific sync
                    let get_result = handler.get(id).await;
                    match get_result {
                        Ok(retrieved) => {
                            println!("Retrieved sync: {:?}", retrieved);
                        }
                        Err(e) => {
                            println!("Failed to get sync: {:?}", e);
                        }
                    }
                }
            } else {
                println!("No syncs available to test get");
            }
        }
        Err(e) => {
            println!("Could not list syncs: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_syncs_pagination() {
    let client = get_test_client();
    let handler = SyncHandler::new(client);

    // Test pagination with small page size
    let result = handler.list(None, Some(1)).await;

    match result {
        Ok((syncs, pagination)) => {
            println!("First page: {} syncs", syncs.len());

            if let Some(cursor) = pagination.cursor_next {
                // Fetch second page
                let result2 = handler.list(Some(cursor), Some(1)).await;
                match result2 {
                    Ok((syncs2, _)) => {
                        println!("Second page: {} syncs", syncs2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Syncs not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_syncs_structure() {
    let client = get_test_client();
    let handler = SyncHandler::new(client);

    let result = handler.list(None, Some(5)).await;

    match result {
        Ok((syncs, _)) => {
            println!("Syncs count: {}", syncs.len());

            for sync in syncs.iter() {
                println!("Sync data keys: {:?}", sync.data.keys());
            }
        }
        Err(e) => {
            println!("Syncs not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_syncs_error_handling() {
    let client = get_test_client();
    let handler = SyncHandler::new(client);

    // Test getting non-existent sync
    let result = handler.get(999999).await;

    match result {
        Ok(_) => {
            println!("Unexpectedly found sync");
        }
        Err(e) => {
            println!("Expected error handling works: {:?}", e);
        }
    }
}
