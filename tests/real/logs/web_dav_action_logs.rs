//! Real API integration tests for WebDavActionLogHandler

use crate::real::*;
use files_sdk::WebDavActionLogHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_web_dav_action_logs() {
    let client = get_test_client();
    let handler = WebDavActionLogHandler::new(client);

    println!("Testing WebDAV action log listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((logs, pagination)) => {
            println!("Successfully listed {} WebDAV action logs", logs.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !logs.is_empty() {
                let first = &logs[0];
                println!(
                    "First WebDAV action log fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No WebDAV action logs available");
            }
        }
        Err(e) => {
            println!("WebDAV action log listing failed: {:?}", e);
        }
    }
}
