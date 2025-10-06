//! Real API integration tests for HistoryExportResultHandler2

use crate::real::*;
use files_sdk::HistoryExportResultHandler2;

#[tokio::test]
async fn test_real_api_get_history_export_result() {
    let client = get_test_client();
    let handler = HistoryExportResultHandler2::new(client);

    // Try to get a specific result (will fail if it doesn't exist)
    let result = handler.get(1).await;

    match result {
        Ok(export_result) => {
            println!("Retrieved history export result: {:?}", export_result);
        }
        Err(e) => {
            println!("Get failed (may not exist): {:?}", e);
        }
    }
}
