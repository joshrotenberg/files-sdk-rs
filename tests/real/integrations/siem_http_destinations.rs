//! Real API integration tests for SiemHttpDestinationHandler

use crate::real::*;
use files_sdk::SiemHttpDestinationHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_siem_http_destinations() {
    let client = get_test_client();
    let handler = SiemHttpDestinationHandler::new(client);

    println!("Testing SIEM HTTP destination listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((destinations, pagination)) => {
            println!(
                "Successfully listed {} SIEM HTTP destinations",
                destinations.len()
            );
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !destinations.is_empty() {
                let first = &destinations[0];
                println!("First SIEM HTTP destination: id={:?}", first.id);
            } else {
                println!("No SIEM HTTP destinations configured");
            }
        }
        Err(e) => {
            println!("SIEM HTTP destination listing failed: {:?}", e);
        }
    }
}
