//! Real API integration tests for MessageCommentHandler

use crate::real::*;
use files_sdk::MessageCommentHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_message_comments() {
    let client = get_test_client();
    let handler = MessageCommentHandler::new(client);

    println!("Testing message comment listing");

    let result = handler.list(None, None, Some(10)).await;

    match result {
        Ok((comments, pagination)) => {
            println!("Successfully listed {} message comments", comments.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !comments.is_empty() {
                let first = &comments[0];
                println!(
                    "First message comment fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No message comments available");
            }
        }
        Err(e) => {
            println!("Message comment listing failed: {:?}", e);
        }
    }
}
