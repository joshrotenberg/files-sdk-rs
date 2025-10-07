//! Real API integration tests for As2IncomingMessageHandler

use crate::real::*;
use files_sdk::As2IncomingMessageHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_as2_incoming_messages() {
    let client = get_test_client();
    let handler = As2IncomingMessageHandler::new(client);

    println!("Testing AS2 incoming message listing");

    let result = handler.list().await;

    match result {
        Ok(messages) => {
            println!(
                "Successfully listed {} AS2 incoming messages",
                messages.len()
            );

            if !messages.is_empty() {
                let first = &messages[0];
                println!(
                    "First AS2 incoming message fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No AS2 incoming messages available");
            }
        }
        Err(e) => {
            println!("AS2 incoming message listing failed: {:?}", e);
        }
    }
}
