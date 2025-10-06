//! Real API integration tests for ActionNotificationExportResultHandler

use crate::real::*;
use files_sdk::ActionNotificationExportResultHandler;

#[tokio::test]
async fn test_real_api_get_action_notification_export_result() {
    let client = get_test_client();
    let handler = ActionNotificationExportResultHandler::new(client);

    // Try to get a specific result (will fail if it doesn't exist)
    let result = handler.get(1).await;

    match result {
        Ok(export_result) => {
            println!(
                "Retrieved action notification export result: {:?}",
                export_result
            );
        }
        Err(e) => {
            println!("Get failed (may not exist): {:?}", e);
        }
    }
}
