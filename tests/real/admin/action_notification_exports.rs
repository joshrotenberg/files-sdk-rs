//! Real API integration tests for ActionNotificationExportHandler

use crate::real::*;
use files_sdk::ActionNotificationExportHandler;

#[tokio::test]
async fn test_real_api_get_action_notification_export() {
    let client = get_test_client();
    let handler = ActionNotificationExportHandler::new(client);

    // Try to get a specific export (will fail if it doesn't exist)
    let result = handler.get(1).await;

    match result {
        Ok(export) => {
            println!("Retrieved action notification export: {:?}", export);
        }
        Err(e) => {
            println!("Get failed (may not exist): {:?}", e);
        }
    }
}
