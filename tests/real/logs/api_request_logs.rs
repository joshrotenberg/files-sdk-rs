//! Real API integration tests for ApiRequestLogHandler

use crate::real::*;
use files_sdk::ApiRequestLogHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_api_request_logs() {
    let client = get_test_client();
    let handler = ApiRequestLogHandler::new(client);

    println!("Testing API request log listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((logs, pagination)) => {
            println!("Successfully listed {} API request logs", logs.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !logs.is_empty() {
                let first = &logs[0];
                println!(
                    "First API request log fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No API request logs available");
            }
        }
        Err(e) => {
            println!("API request log listing failed: {:?}", e);
        }
    }
}
