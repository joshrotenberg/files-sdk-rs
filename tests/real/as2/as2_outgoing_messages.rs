//! Real API integration tests for As2OutgoingMessageHandler

use crate::real::*;
use files_sdk::As2OutgoingMessageHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_as2_outgoing_messages() {
    let client = get_test_client();
    let handler = As2OutgoingMessageHandler::new(client);

    println!("Testing AS2 outgoing message listing");

    let result = handler.list().await;

    match result {
        Ok(messages) => {
            println!(
                "Successfully listed {} AS2 outgoing messages",
                messages.len()
            );

            if !messages.is_empty() {
                let first = &messages[0];
                println!(
                    "First AS2 outgoing message fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No AS2 outgoing messages available");
            }
        }
        Err(e) => {
            println!("AS2 outgoing message listing failed: {:?}", e);
        }
    }
}
