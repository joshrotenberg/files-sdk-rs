//! Real API integration tests for AutomationLogHandler

use crate::real::*;
use files_sdk::AutomationLogHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_automation_logs() {
    let client = get_test_client();
    let handler = AutomationLogHandler::new(client);

    println!("Testing automation log listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((logs, pagination)) => {
            println!("Successfully listed {} automation logs", logs.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !logs.is_empty() {
                let first = &logs[0];
                println!(
                    "First automation log fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No automation logs available");
            }
        }
        Err(e) => {
            println!("Automation log listing failed: {:?}", e);
        }
    }
}
