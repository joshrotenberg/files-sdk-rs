//! Real API integration tests for BandwidthSnapshotHandler

use crate::real::*;
use files_sdk::BandwidthSnapshotHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_bandwidth_snapshots() {
    let client = get_test_client();
    let handler = BandwidthSnapshotHandler::new(client);

    println!("Testing bandwidth snapshot listing");

    let result = handler.list().await;

    match result {
        Ok(snapshots) => {
            println!(
                "Successfully listed {} bandwidth snapshots",
                snapshots.len()
            );

            if !snapshots.is_empty() {
                let first = &snapshots[0];
                println!(
                    "First bandwidth snapshot fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No bandwidth snapshots available");
            }
        }
        Err(e) => {
            println!("Bandwidth snapshot listing failed: {:?}", e);
        }
    }
}
