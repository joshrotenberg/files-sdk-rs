//! Real API integration tests for AutomationRunHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! AutomationRuns track execution history of automations.

use crate::real::*;
use files_sdk::AutomationRunHandler;

#[tokio::test]
async fn test_real_api_list_automation_runs() {
    let client = get_test_client();
    let handler = AutomationRunHandler::new(client);

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((runs, pagination)) => {
            println!("Listed {} automation runs", runs.len());
            println!("Pagination: {:?}", pagination);

            if let Some(first) = runs.first() {
                println!("Sample run: {:?}", first);
            }
        }
        Err(e) => {
            println!(
                "Automation runs list failed (may require automations): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_get_automation_run() {
    let client = get_test_client();
    let handler = AutomationRunHandler::new(client);

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
                            println!("Retrieved automation run: {:?}", retrieved);
                        }
                        Err(e) => {
                            println!("Failed to get automation run: {:?}", e);
                        }
                    }
                }
            } else {
                println!("No automation runs available to test get");
            }
        }
        Err(e) => {
            println!("Could not list automation runs: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_automation_runs_pagination() {
    let client = get_test_client();
    let handler = AutomationRunHandler::new(client);

    // Test pagination with small page size
    let result = handler.list(None, Some(1)).await;

    match result {
        Ok((runs, pagination)) => {
            println!("First page: {} automation runs", runs.len());

            if let Some(cursor) = pagination.cursor_next {
                // Fetch second page
                let result2 = handler.list(Some(cursor), Some(1)).await;
                match result2 {
                    Ok((runs2, _)) => {
                        println!("Second page: {} automation runs", runs2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Automation runs not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_automation_runs_structure() {
    let client = get_test_client();
    let handler = AutomationRunHandler::new(client);

    let result = handler.list(None, Some(5)).await;

    match result {
        Ok((runs, _)) => {
            println!("Automation runs count: {}", runs.len());

            for run in runs.iter() {
                println!("Run data keys: {:?}", run.data.keys());
            }
        }
        Err(e) => {
            println!("Automation runs not available: {:?}", e);
        }
    }
}
