//! Real API integration tests for ClickwrapHandler

use crate::real::*;
use files_sdk::ClickwrapHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_clickwraps() {
    let client = get_test_client();
    let handler = ClickwrapHandler::new(client);

    println!("Testing clickwrap listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((clickwraps, pagination)) => {
            println!("Successfully listed {} clickwraps", clickwraps.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !clickwraps.is_empty() {
                let first = &clickwraps[0];
                println!(
                    "First clickwrap: id={:?}, fields={:?}",
                    first.id,
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No clickwraps found");
            }
        }
        Err(e) => {
            println!("Clickwrap listing failed: {:?}", e);
        }
    }
}
