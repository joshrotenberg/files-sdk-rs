//! Real API integration tests for As2StationHandler

use crate::real::*;
use files_sdk::As2StationHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_as2_stations() {
    let client = get_test_client();
    let handler = As2StationHandler::new(client);

    println!("Testing AS2 station listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((stations, pagination)) => {
            println!("Successfully listed {} AS2 stations", stations.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !stations.is_empty() {
                let first = &stations[0];
                println!("First AS2 station: id={:?}", first.id);
            } else {
                println!("No AS2 stations configured");
            }
        }
        Err(e) => {
            println!("AS2 station listing failed: {:?}", e);
        }
    }
}
