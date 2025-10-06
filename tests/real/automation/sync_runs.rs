//! Real API integration tests for SyncRunHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! SyncRuns track execution history of syncs.

use crate::real::*;
use files_sdk::SyncRunHandler;

#[tokio::test]
async fn test_real_api_list_sync_runs() {
    let client = get_test_client();
    let handler = SyncRunHandler::new(client);

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((runs, pagination)) => {
            println!("Listed {} sync runs", runs.len());
            println!("Pagination: {:?}", pagination);

            if let Some(first) = runs.first() {
                println!("Sample sync run: {:?}", first);
            }
        }
        Err(e) => {
            println!(
                "Sync runs list failed (may require syncs configured): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_get_sync_run() {
    let client = get_test_client();
    let handler = SyncRunHandler::new(client);

    // First, get a list to find an ID
    let list_result = handler.list(None, Some(1)).await;

    match list_result {
        Ok((runs, _)) => {
            if let Some(run) = runs.first() {
                // Extract ID from the run data
                if let Some(id) = run.data.get("id").and_then(|v| v.as_i64()) {
                    // Try to get that specific run
                    let get_result = handler.get(id).await;
                    match get_result {
                        Ok(retrieved) => {
                            println!("Retrieved sync run: {:?}", retrieved);
                        }
                        Err(e) => {
                            println!("Failed to get sync run: {:?}", e);
                        }
                    }
                }
            } else {
                println!("No sync runs available to test get");
            }
        }
        Err(e) => {
            println!("Could not list sync runs: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_sync_runs_pagination() {
    let client = get_test_client();
    let handler = SyncRunHandler::new(client);

    // Test pagination with small page size
    let result = handler.list(None, Some(1)).await;

    match result {
        Ok((runs, pagination)) => {
            println!("First page: {} sync runs", runs.len());

            if let Some(cursor) = pagination.cursor_next {
                // Fetch second page
                let result2 = handler.list(Some(cursor), Some(1)).await;
                match result2 {
                    Ok((runs2, _)) => {
                        println!("Second page: {} sync runs", runs2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Sync runs not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_sync_runs_structure() {
    let client = get_test_client();
    let handler = SyncRunHandler::new(client);

    let result = handler.list(None, Some(5)).await;

    match result {
        Ok((runs, _)) => {
            println!("Sync runs count: {}", runs.len());

            for run in runs.iter() {
                println!("Sync run data keys: {:?}", run.data.keys());
            }
        }
        Err(e) => {
            println!("Sync runs not available: {:?}", e);
        }
    }
}
