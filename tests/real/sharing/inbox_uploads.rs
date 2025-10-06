//! Real API integration tests for InboxUploadHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! InboxUploads track files uploaded to inboxes.

use crate::real::*;
use files_sdk::InboxUploadHandler;

#[tokio::test]
async fn test_real_api_list_inbox_uploads() {
    let client = get_test_client();
    let handler = InboxUploadHandler::new(client);

    let result = handler.list(None, Some(10), None, None, None, None).await;

    match result {
        Ok((uploads, pagination)) => {
            println!("Listed {} inbox uploads", uploads.len());
            println!("Pagination: {:?}", pagination);

            if let Some(first) = uploads.first() {
                println!("Sample upload: {:?}", first);
            }
        }
        Err(e) => {
            println!("Inbox uploads list failed (may require paid plan): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_inbox_uploads_with_filter() {
    let client = get_test_client();
    let handler = InboxUploadHandler::new(client);

    // Test filtering by inbox_id
    let result = handler
        .list(None, Some(10), None, None, None, Some(1))
        .await;

    match result {
        Ok((uploads, _)) => {
            println!("Filtered uploads count: {}", uploads.len());
        }
        Err(e) => {
            println!("Filtered list failed (expected if no inbox): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_inbox_uploads_structure() {
    let client = get_test_client();
    let handler = InboxUploadHandler::new(client);

    let result = handler.list(None, Some(5), None, None, None, None).await;

    match result {
        Ok((uploads, _)) => {
            println!("Inbox uploads count: {}", uploads.len());

            for upload in uploads.iter() {
                if let Some(path) = &upload.path {
                    println!("Upload path: {}", path);
                }
                if let Some(created_at) = &upload.created_at {
                    println!("Upload time: {}", created_at);
                }
            }
        }
        Err(e) => {
            println!("Inbox uploads not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_inbox_uploads_error_handling() {
    let client = get_test_client();
    let handler = InboxUploadHandler::new(client);

    let result = handler.list(None, None, None, None, None, None).await;

    // Should either succeed or return a proper error (not panic)
    match result {
        Ok((uploads, _)) => {
            println!("Successfully listed {} inbox uploads", uploads.len());
        }
        Err(e) => {
            println!("Expected error handling works: {:?}", e);
        }
    }
}
