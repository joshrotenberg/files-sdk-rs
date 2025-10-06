//! Real API integration tests for HistoryExportHandler2

use crate::real::*;
use files_sdk::HistoryExportHandler2;

#[tokio::test]
async fn test_real_api_get_history_export() {
    let client = get_test_client();
    let handler = HistoryExportHandler2::new(client);

    // Try to get a specific export (will fail if it doesn't exist)
    let result = handler.get(1).await;

    match result {
        Ok(export) => {
            println!("Retrieved history export: {:?}", export);
        }
        Err(e) => {
            println!("Get failed (may not exist): {:?}", e);
        }
    }
}
