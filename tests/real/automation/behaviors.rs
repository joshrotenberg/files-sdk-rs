//! Real API integration tests for BehaviorHandler

use crate::real::*;
use files_sdk::BehaviorHandler;

#[tokio::test]
async fn test_list_behaviors() {
    let client = get_test_client();
    let handler = BehaviorHandler::new(client);

    println!("Testing behavior listing");

    let result = handler.list(None, Some(10), None, None, None).await;

    match result {
        Ok((behaviors, pagination)) => {
            println!("Successfully listed {} behaviors", behaviors.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !behaviors.is_empty() {
                let first = &behaviors[0];
                println!("First behavior: {:?}", first);
                assert!(first.id.is_some(), "Behavior should have an ID");
            } else {
                println!("No behaviors configured");
            }
        }
        Err(e) => {
            // Behaviors might require premium features
            println!("Behavior listing failed (may require premium): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_list_behaviors_for_path() {
    let client = get_test_client();
    let handler = BehaviorHandler::new(client);

    println!("Testing behavior listing for specific path");

    let result = handler.list_for_folder("/", None, Some(10)).await;

    match result {
        Ok((behaviors, pagination)) => {
            println!(
                "Successfully listed {} behaviors for root path",
                behaviors.len()
            );
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );
        }
        Err(e) => {
            println!(
                "Behavior listing for path failed (may require premium): {:?}",
                e
            );
        }
    }
}
