//! Real API integration tests for UsageSnapshotHandler

use crate::real::*;
use files_sdk::UsageSnapshotHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_usage_snapshots() {
    let client = get_test_client();
    let handler = UsageSnapshotHandler::new(client);

    println!("Testing usage snapshot listing");

    let result = handler.list().await;

    match result {
        Ok(snapshots) => {
            println!("Successfully listed {} usage snapshots", snapshots.len());

            if !snapshots.is_empty() {
                let first = &snapshots[0];
                println!(
                    "First usage snapshot fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No usage snapshots available");
            }
        }
        Err(e) => {
            println!("Usage snapshot listing failed: {:?}", e);
        }
    }
}
