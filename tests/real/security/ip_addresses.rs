//! Real API integration tests for IpAddressHandler

use crate::real::*;
use files_sdk::IpAddressHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_ip_addresses() {
    let client = get_test_client();
    let handler = IpAddressHandler::new(client);

    println!("Testing IP address listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((ip_addresses, pagination)) => {
            println!("Successfully listed {} IP addresses", ip_addresses.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !ip_addresses.is_empty() {
                let first = &ip_addresses[0];
                println!(
                    "First IP address fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No IP addresses found");
            }
        }
        Err(e) => {
            println!("IP address listing failed: {:?}", e);
        }
    }
}
