//! Real API integration tests for As2PartnerHandler

use crate::real::*;
use files_sdk::As2PartnerHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_as2_partners() {
    let client = get_test_client();
    let handler = As2PartnerHandler::new(client);

    println!("Testing AS2 partner listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((partners, pagination)) => {
            println!("Successfully listed {} AS2 partners", partners.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !partners.is_empty() {
                let first = &partners[0];
                println!("First AS2 partner: id={:?}", first.id);
            } else {
                println!("No AS2 partners configured");
            }
        }
        Err(e) => {
            println!("AS2 partner listing failed: {:?}", e);
        }
    }
}
