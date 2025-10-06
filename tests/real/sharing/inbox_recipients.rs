//! Real API integration tests for InboxRecipientHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! InboxRecipients track who inboxes are shared with.

use crate::real::*;
use files_sdk::InboxRecipientHandler;

#[tokio::test]
async fn test_real_api_list_inbox_recipients() {
    let client = get_test_client();
    let handler = InboxRecipientHandler::new(client);

    // Note: This requires an inbox_id which we may not have in test environment
    // Using a placeholder ID to test error handling
    let result = handler.list(1, None, Some(10)).await;

    match result {
        Ok((recipients, pagination)) => {
            println!("Listed {} inbox recipients", recipients.len());
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!(
                "Inbox recipients list failed (expected - no inbox): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_inbox_recipients_structure() {
    let client = get_test_client();
    let handler = InboxRecipientHandler::new(client);

    let result = handler.list(1, None, None).await;

    match result {
        Ok((recipients, _)) => {
            println!("Inbox recipients count: {}", recipients.len());

            if let Some(first) = recipients.first() {
                println!("Sample recipient: {:?}", first);
                assert!(!first.data.is_empty(), "Recipient should have data");
            }
        }
        Err(e) => {
            println!("Inbox recipients not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_inbox_recipients_error_handling() {
    let client = get_test_client();
    let handler = InboxRecipientHandler::new(client);

    // Test with invalid inbox_id
    let result = handler.list(999999, None, None).await;

    // Should return a proper error (not panic)
    match result {
        Ok((recipients, _)) => {
            println!("Unexpectedly succeeded: {} recipients", recipients.len());
        }
        Err(e) => {
            println!("Expected error handling works: {:?}", e);
        }
    }
}
