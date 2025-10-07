//! Real API integration tests for EmailLogHandler

use crate::real::*;
use files_sdk::EmailLogHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_email_logs() {
    let client = get_test_client();
    let handler = EmailLogHandler::new(client);

    println!("Testing email log listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((logs, pagination)) => {
            println!("Successfully listed {} email logs", logs.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !logs.is_empty() {
                let first = &logs[0];
                println!(
                    "First email log fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No email logs available");
            }
        }
        Err(e) => {
            println!("Email log listing failed: {:?}", e);
        }
    }
}
