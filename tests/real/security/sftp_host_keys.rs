//! Real API integration tests for SftpHostKeyHandler

use crate::real::*;
use files_sdk::SftpHostKeyHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_sftp_host_keys() {
    let client = get_test_client();
    let handler = SftpHostKeyHandler::new(client);

    println!("Testing SFTP host key listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((keys, pagination)) => {
            println!("Successfully listed {} SFTP host keys", keys.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !keys.is_empty() {
                let first = &keys[0];
                println!(
                    "First SFTP host key: id={:?}, fields={:?}",
                    first.id,
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No SFTP host keys found");
            }
        }
        Err(e) => {
            println!("SFTP host key listing failed: {:?}", e);
        }
    }
}
