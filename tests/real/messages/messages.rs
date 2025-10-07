//! Real API integration tests for MessageHandler

use crate::real::*;
use files_sdk::MessageHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_messages() {
    let client = get_test_client();
    let handler = MessageHandler::new(client);

    println!("Testing message listing");

    let result = handler.list(None, Some(10), None).await;

    match result {
        Ok((messages, pagination)) => {
            println!("Successfully listed {} messages", messages.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !messages.is_empty() {
                let first = &messages[0];
                println!("First message: id={:?}", first.id);
            } else {
                println!("No messages available");
            }
        }
        Err(e) => {
            println!("Message listing failed: {:?}", e);
        }
    }
}
