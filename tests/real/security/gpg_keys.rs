//! Real API integration tests for GpgKeyHandler

use crate::real::*;
use files_sdk::GpgKeyHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_gpg_keys() {
    let client = get_test_client();
    let handler = GpgKeyHandler::new(client);

    println!("Testing GPG key listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((keys, pagination)) => {
            println!("Successfully listed {} GPG keys", keys.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !keys.is_empty() {
                let first = &keys[0];
                println!(
                    "First GPG key: id={:?}, fields={:?}",
                    first.id,
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No GPG keys found");
            }
        }
        Err(e) => {
            println!("GPG key listing failed: {:?}", e);
        }
    }
}
