//! Real API integration tests for ExavaultApiRequestLogHandler

use crate::real::*;
use files_sdk::ExavaultApiRequestLogHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_exavault_api_request_logs() {
    let client = get_test_client();
    let handler = ExavaultApiRequestLogHandler::new(client);

    println!("Testing ExaVault API request log listing");

    let result = handler.list().await;

    match result {
        Ok(logs) => {
            println!(
                "Successfully listed {} ExaVault API request logs",
                logs.len()
            );

            if !logs.is_empty() {
                let first = &logs[0];
                println!(
                    "First ExaVault API request log fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No ExaVault API request logs available");
            }
        }
        Err(e) => {
            println!("ExaVault API request log listing failed: {:?}", e);
        }
    }
}
