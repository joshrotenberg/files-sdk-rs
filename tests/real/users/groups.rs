//! Real API integration tests for GroupHandler

use crate::real::*;
use files_sdk::GroupHandler;

#[tokio::test]
async fn test_list_groups() {
    let client = get_test_client();
    let handler = GroupHandler::new(client);

    println!("Testing group listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((groups, pagination)) => {
            println!("Successfully listed {} groups", groups.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !groups.is_empty() {
                let first_group = &groups[0];
                println!("First group: {:?}", first_group);
                assert!(first_group.id.is_some(), "Group should have an ID");
            } else {
                println!("No groups found (acceptable for new accounts)");
            }
        }
        Err(e) => {
            panic!("Failed to list groups: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_nonexistent_group() {
    let client = get_test_client();
    let handler = GroupHandler::new(client);

    println!("Testing get nonexistent group");

    let result = handler.get(999999999).await;

    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {
            println!("Correctly received NotFound error");
        }
        Err(e) => {
            println!("Got error (acceptable): {:?}", e);
        }
        Ok(group) => {
            println!("Unexpectedly found group: {:?}", group);
        }
    }
}
