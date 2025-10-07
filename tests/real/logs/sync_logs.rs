//! Real API integration tests for SyncLogHandler

use crate::real::*;
use files_sdk::SyncLogHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_sync_logs() {
    let client = get_test_client();
    let handler = SyncLogHandler::new(client);

    println!("Testing sync log listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((logs, pagination)) => {
            println!("Successfully listed {} sync logs", logs.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !logs.is_empty() {
                let first = &logs[0];
                println!(
                    "First sync log fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No sync logs available");
            }
        }
        Err(e) => {
            println!("Sync log listing failed: {:?}", e);
        }
    }
}
