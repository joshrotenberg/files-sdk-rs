//! Real API integration tests for PriorityHandler

use crate::real::*;
use files_sdk::PriorityHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_priorities() {
    let client = get_test_client();
    let handler = PriorityHandler::new(client);

    println!("Testing priority listing");

    let result = handler.list("/").await;

    match result {
        Ok(priorities) => {
            println!("Successfully listed {} priorities", priorities.len());

            if !priorities.is_empty() {
                let first = &priorities[0];
                println!(
                    "First priority fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No priorities found");
            }
        }
        Err(e) => {
            println!("Priority listing failed: {:?}", e);
        }
    }
}
