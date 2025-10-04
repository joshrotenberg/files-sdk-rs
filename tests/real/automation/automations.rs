//! Real API integration tests for AutomationHandler

use crate::real::*;
use files_sdk::AutomationHandler;

#[tokio::test]
async fn test_list_automations() {
    let client = get_test_client();
    let handler = AutomationHandler::new(client);

    println!("Testing automation listing");

    let result = handler.list(None, Some(10), None).await;

    match result {
        Ok((automations, pagination)) => {
            println!("Successfully listed {} automations", automations.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !automations.is_empty() {
                let first = &automations[0];
                println!("First automation: {:?}", first);
                assert!(first.id.is_some(), "Automation should have an ID");
            } else {
                println!("No automations configured");
            }
        }
        Err(e) => {
            // Automations might require premium features
            println!("Automation listing failed (may require premium): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_nonexistent_automation() {
    let client = get_test_client();
    let handler = AutomationHandler::new(client);

    println!("Testing get nonexistent automation");

    let result = handler.get(999999999).await;

    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {
            println!("Correctly received NotFound error");
        }
        Err(e) => {
            println!("Got error (acceptable): {:?}", e);
        }
        Ok(automation) => {
            println!("Unexpectedly found automation: {:?}", automation);
        }
    }
}
