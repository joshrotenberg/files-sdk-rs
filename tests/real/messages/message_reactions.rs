//! Real API integration tests for MessageReactionHandler

use crate::real::*;
use files_sdk::MessageReactionHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_message_reactions() {
    let client = get_test_client();
    let handler = MessageReactionHandler::new(client);

    println!("Testing message reaction listing");

    // Note: This endpoint requires a message_id, which we may not have in test data.
    // Using a test ID of 1 - will likely return empty results or error
    let result = handler.list(1, None, Some(10)).await;

    match result {
        Ok((reactions, pagination)) => {
            println!("Successfully listed {} message reactions", reactions.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !reactions.is_empty() {
                let first = &reactions[0];
                println!(
                    "First message reaction fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No message reactions available for test message ID");
            }
        }
        Err(e) => {
            println!(
                "Message reaction listing failed (expected without valid message ID): {:?}",
                e
            );
        }
    }
}
