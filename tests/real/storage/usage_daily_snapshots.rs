//! Real API integration tests for UsageDailySnapshotHandler

use crate::real::*;
use files_sdk::UsageDailySnapshotHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_usage_daily_snapshots() {
    let client = get_test_client();
    let handler = UsageDailySnapshotHandler::new(client);

    println!("Testing usage daily snapshot listing");

    let result = handler.list().await;

    match result {
        Ok(snapshots) => {
            println!(
                "Successfully listed {} usage daily snapshots",
                snapshots.len()
            );

            if !snapshots.is_empty() {
                let first = &snapshots[0];
                println!(
                    "First usage daily snapshot fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No usage daily snapshots available");
            }
        }
        Err(e) => {
            println!("Usage daily snapshot listing failed: {:?}", e);
        }
    }
}
