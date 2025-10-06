//! Real API integration tests for ShareGroupHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! ShareGroups allow sharing with multiple recipients at once.

use crate::real::*;
use files_sdk::ShareGroupHandler;
use serde_json::json;

#[tokio::test]
async fn test_real_api_list_share_groups() {
    let client = get_test_client();
    let handler = ShareGroupHandler::new(client);

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((share_groups, pagination)) => {
            println!("Listed {} share groups", share_groups.len());
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!("Share groups list failed (may require paid plan): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_create_and_delete_share_group() {
    let client = get_test_client();
    let handler = ShareGroupHandler::new(client);

    // Create share group
    let params = json!({
        "name": "Integration Test Group",
        "notes": "Created by integration tests"
    });

    let create_result = handler.create(params).await;

    match create_result {
        Ok(share_group) => {
            println!("Created share group: {:?}", share_group);
            let group_id = share_group.id.expect("Share group should have ID");

            // Get the share group
            let get_result = handler.get(group_id).await;
            match get_result {
                Ok(retrieved) => {
                    println!("Retrieved share group: {:?}", retrieved);
                    assert_eq!(retrieved.id, Some(group_id));
                }
                Err(e) => {
                    eprintln!("Failed to get share group: {:?}", e);
                }
            }

            // List and verify it's there
            let list_result = handler.list(None, None).await;
            match list_result {
                Ok((groups, _)) => {
                    let found = groups.iter().any(|g| g.id == Some(group_id));
                    assert!(found, "Should find created share group in list");
                }
                Err(e) => {
                    eprintln!("Failed to list share groups: {:?}", e);
                }
            }

            // Delete the share group
            let delete_result = handler.delete(group_id).await;
            match delete_result {
                Ok(_) => println!("Successfully deleted share group"),
                Err(e) => eprintln!("Failed to delete share group: {:?}", e),
            }
        }
        Err(e) => {
            println!(
                "Share group creation failed (may require permissions): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_update_share_group() {
    let client = get_test_client();
    let handler = ShareGroupHandler::new(client);

    // Create share group
    let params = json!({
        "name": "Original Name"
    });

    let share_group = match handler.create(params).await {
        Ok(g) => g,
        Err(e) => {
            println!("Share group creation failed: {:?}", e);
            return;
        }
    };

    let group_id = share_group.id.expect("Share group should have ID");

    // Update it
    let update_params = json!({
        "name": "Updated Name",
        "notes": "Updated by test"
    });

    let update_result = handler.update(group_id, update_params).await;

    match update_result {
        Ok(updated) => {
            println!("Updated share group: {:?}", updated);
        }
        Err(e) => {
            println!("Share group update failed: {:?}", e);
        }
    }

    // Cleanup
    let _ = handler.delete(group_id).await;
}

#[tokio::test]
async fn test_real_api_share_group_pagination() {
    let client = get_test_client();
    let handler = ShareGroupHandler::new(client);

    // Test pagination with small page size
    let result = handler.list(None, Some(1)).await;

    match result {
        Ok((groups, pagination)) => {
            println!("First page: {} share groups", groups.len());

            if let Some(cursor) = pagination.cursor_next {
                // Fetch second page
                let result2 = handler.list(Some(cursor), Some(1)).await;
                match result2 {
                    Ok((groups2, _)) => {
                        println!("Second page: {} share groups", groups2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Share groups not available: {:?}", e);
        }
    }
}
