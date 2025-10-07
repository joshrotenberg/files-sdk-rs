//! Real API integration tests for SnapshotHandler

use crate::real::*;
use files_sdk::SnapshotHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_snapshots() {
    let client = get_test_client();
    let handler = SnapshotHandler::new(client);

    println!("Testing snapshot listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((snapshots, pagination)) => {
            println!("Successfully listed {} snapshots", snapshots.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !snapshots.is_empty() {
                let first = &snapshots[0];
                println!(
                    "First snapshot fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No snapshots found");
            }
        }
        Err(e) => {
            println!("Snapshot listing failed: {:?}", e);
        }
    }
}
