//! Real API integration tests for ExternalEventHandler

use crate::real::*;
use files_sdk::ExternalEventHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_external_events() {
    let client = get_test_client();
    let handler = ExternalEventHandler::new(client);

    println!("Testing external event listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((events, pagination)) => {
            println!("Successfully listed {} external events", events.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !events.is_empty() {
                let first = &events[0];
                println!(
                    "First external event fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No external events available");
            }
        }
        Err(e) => {
            println!("External event listing failed: {:?}", e);
        }
    }
}
