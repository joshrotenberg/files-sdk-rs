//! Real API integration tests for PublicHostingRequestLogHandler

use crate::real::*;
use files_sdk::PublicHostingRequestLogHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_public_hosting_request_logs() {
    let client = get_test_client();
    let handler = PublicHostingRequestLogHandler::new(client);

    println!("Testing public hosting request log listing");

    let result = handler.list().await;

    match result {
        Ok(logs) => {
            println!(
                "Successfully listed {} public hosting request logs",
                logs.len()
            );

            if !logs.is_empty() {
                let first = &logs[0];
                println!(
                    "First public hosting request log fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No public hosting request logs available");
            }
        }
        Err(e) => {
            println!("Public hosting request log listing failed: {:?}", e);
        }
    }
}
