//! Real API integration tests for FtpActionLogHandler

use crate::real::*;
use files_sdk::FtpActionLogHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_ftp_action_logs() {
    let client = get_test_client();
    let handler = FtpActionLogHandler::new(client);

    println!("Testing FTP action log listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((logs, pagination)) => {
            println!("Successfully listed {} FTP action logs", logs.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !logs.is_empty() {
                let first = &logs[0];
                println!(
                    "First FTP action log fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No FTP action logs available");
            }
        }
        Err(e) => {
            println!("FTP action log listing failed: {:?}", e);
        }
    }
}
