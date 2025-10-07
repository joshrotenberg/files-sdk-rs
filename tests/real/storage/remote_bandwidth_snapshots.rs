//! Real API integration tests for RemoteBandwidthSnapshotHandler

use crate::real::*;
use files_sdk::RemoteBandwidthSnapshotHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_remote_bandwidth_snapshots() {
    let client = get_test_client();
    let handler = RemoteBandwidthSnapshotHandler::new(client);

    println!("Testing remote bandwidth snapshot listing");

    let result = handler.list().await;

    match result {
        Ok(snapshots) => {
            println!(
                "Successfully listed {} remote bandwidth snapshots",
                snapshots.len()
            );

            if !snapshots.is_empty() {
                let first = &snapshots[0];
                println!(
                    "First remote bandwidth snapshot fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No remote bandwidth snapshots available");
            }
        }
        Err(e) => {
            println!("Remote bandwidth snapshot listing failed: {:?}", e);
        }
    }
}
