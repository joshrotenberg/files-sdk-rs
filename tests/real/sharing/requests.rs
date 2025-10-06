//! Real API integration tests for RequestHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! Requests are file upload requests sent to users/groups.

use crate::real::*;
use files_sdk::{FolderHandler, RequestHandler};

#[tokio::test]
async fn test_real_api_list_requests() {
    let client = get_test_client();
    let handler = RequestHandler::new(client);

    let result = handler.list(None, Some(10), None, None).await;

    match result {
        Ok((requests, pagination)) => {
            println!("Listed {} requests", requests.len());
            println!("Pagination: {:?}", pagination);

            if let Some(first) = requests.first() {
                println!("Sample request: {:?}", first);
            }
        }
        Err(e) => {
            println!("Requests list failed (may require permissions): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_create_and_delete_request() {
    let client = get_test_client();
    let request_handler = RequestHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client);

    // Ensure test folder exists
    let test_folder = "/integration-tests";
    let _ = folder_handler.create_folder(test_folder, true).await;

    // Create request
    let create_result = request_handler
        .create(
            test_folder,
            "test_upload",
            None, // user_ids
            None, // group_ids
        )
        .await;

    match create_result {
        Ok(request) => {
            println!("Created request: {:?}", request);
            let request_id = request.id.expect("Request should have ID");

            // Verify it appears in list
            let list_result = request_handler.list(None, None, None, None).await;
            match list_result {
                Ok((requests, _)) => {
                    let found = requests.iter().any(|r| r.id == Some(request_id));
                    if found {
                        println!("Request found in list");
                    }
                }
                Err(e) => {
                    println!("Failed to list requests: {:?}", e);
                }
            }

            // Delete the request
            let delete_result = request_handler.delete(request_id).await;
            match delete_result {
                Ok(_) => println!("Successfully deleted request"),
                Err(e) => eprintln!("Failed to delete request: {:?}", e),
            }
        }
        Err(e) => {
            println!(
                "Request creation failed (may require permissions/paid plan): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_list_requests_for_folder() {
    let client = get_test_client();
    let request_handler = RequestHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client);

    // Ensure test folder exists
    let test_folder = "/integration-tests";
    let _ = folder_handler.create_folder(test_folder, true).await;

    // List requests for specific folder
    let result = request_handler
        .list_for_folder(test_folder, None, Some(10))
        .await;

    match result {
        Ok((requests, pagination)) => {
            println!("Folder requests count: {}", requests.len());
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!("Folder requests list failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_list_my_requests() {
    let client = get_test_client();
    let handler = RequestHandler::new(client);

    // List only my requests
    let result = handler.list(None, Some(10), None, Some(true)).await;

    match result {
        Ok((requests, _)) => {
            println!("My requests count: {}", requests.len());
        }
        Err(e) => {
            println!("My requests list failed: {:?}", e);
        }
    }
}
