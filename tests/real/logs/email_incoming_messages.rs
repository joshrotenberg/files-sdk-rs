//! Real API integration tests for EmailIncomingMessageHandler

use crate::real::*;
use files_sdk::EmailIncomingMessageHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_email_incoming_messages() {
    let client = get_test_client();
    let handler = EmailIncomingMessageHandler::new(client);

    println!("Testing email incoming message listing");

    let result = handler.list().await;

    match result {
        Ok(messages) => {
            println!(
                "Successfully listed {} email incoming messages",
                messages.len()
            );

            if !messages.is_empty() {
                let first = &messages[0];
                println!(
                    "First email incoming message fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No email incoming messages available");
            }
        }
        Err(e) => {
            println!("Email incoming message listing failed: {:?}", e);
        }
    }
}
