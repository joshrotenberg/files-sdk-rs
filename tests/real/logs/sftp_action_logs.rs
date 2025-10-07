//! Real API integration tests for SftpActionLogHandler

use crate::real::*;
use files_sdk::SftpActionLogHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_sftp_action_logs() {
    let client = get_test_client();
    let handler = SftpActionLogHandler::new(client);

    println!("Testing SFTP action log listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((logs, pagination)) => {
            println!("Successfully listed {} SFTP action logs", logs.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !logs.is_empty() {
                let first = &logs[0];
                println!(
                    "First SFTP action log fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No SFTP action logs available");
            }
        }
        Err(e) => {
            println!("SFTP action log listing failed: {:?}", e);
        }
    }
}
