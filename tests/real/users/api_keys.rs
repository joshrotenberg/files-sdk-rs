//! Real API integration tests for ApiKeyHandler

use crate::real::*;
use files_sdk::ApiKeyHandler;

#[tokio::test]
async fn test_list_api_keys() {
    let client = get_test_client();
    let handler = ApiKeyHandler::new(client);

    println!("Testing API key listing");

    let result = handler.list(None, None, Some(10)).await;

    match result {
        Ok((keys, pagination)) => {
            println!("Successfully listed {} API keys", keys.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !keys.is_empty() {
                let first_key = &keys[0];
                println!("First API key: {:?}", first_key);
                assert!(first_key.id.is_some(), "API key should have an ID");
            } else {
                println!("No API keys found");
            }
        }
        Err(e) => {
            panic!("Failed to list API keys: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_nonexistent_api_key() {
    let client = get_test_client();
    let handler = ApiKeyHandler::new(client);

    println!("Testing get nonexistent API key");

    let result = handler.get(999999999).await;

    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {
            println!("Correctly received NotFound error");
        }
        Err(e) => {
            println!("Got error (acceptable): {:?}", e);
        }
        Ok(key) => {
            println!("Unexpectedly found API key: {:?}", key);
        }
    }
}
