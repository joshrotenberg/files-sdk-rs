//! Real API integration tests for NotificationHandler

use crate::real::*;
use files_sdk::NotificationHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_notifications() {
    let client = get_test_client();
    let handler = NotificationHandler::new(client);

    println!("Testing notification listing");

    let result = handler.list(None, Some(10), None, None).await;

    match result {
        Ok((notifications, pagination)) => {
            println!("Successfully listed {} notifications", notifications.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !notifications.is_empty() {
                let first = &notifications[0];
                println!("First notification: id={:?}", first.id);
            } else {
                println!("No notifications available");
            }
        }
        Err(e) => {
            println!("Notification listing failed: {:?}", e);
        }
    }
}
