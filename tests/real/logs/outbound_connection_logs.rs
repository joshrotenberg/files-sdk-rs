//! Real API integration tests for OutboundConnectionLogHandler

use crate::real::*;
use files_sdk::OutboundConnectionLogHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_outbound_connection_logs() {
    let client = get_test_client();
    let handler = OutboundConnectionLogHandler::new(client);

    println!("Testing outbound connection log listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((logs, pagination)) => {
            println!(
                "Successfully listed {} outbound connection logs",
                logs.len()
            );
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !logs.is_empty() {
                let first = &logs[0];
                println!(
                    "First outbound connection log fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No outbound connection logs available");
            }
        }
        Err(e) => {
            println!("Outbound connection log listing failed: {:?}", e);
        }
    }
}
