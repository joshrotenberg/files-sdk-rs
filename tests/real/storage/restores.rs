//! Real API integration tests for RestoreHandler

use crate::real::*;
use files_sdk::RestoreHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_restores() {
    let client = get_test_client();
    let handler = RestoreHandler::new(client);

    println!("Testing restore listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((restores, pagination)) => {
            println!("Successfully listed {} restores", restores.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !restores.is_empty() {
                let first = &restores[0];
                println!(
                    "First restore fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No restores found");
            }
        }
        Err(e) => {
            println!("Restore listing failed: {:?}", e);
        }
    }
}
