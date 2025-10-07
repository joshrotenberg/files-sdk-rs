//! Real API integration tests for LockHandler

use crate::real::*;
use files_sdk::LockHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_locks() {
    let client = get_test_client();
    let handler = LockHandler::new(client);

    println!("Testing lock listing");

    let result = handler.list_for_path("/", false).await;

    match result {
        Ok(locks) => {
            println!("Successfully listed {} locks", locks.len());

            if !locks.is_empty() {
                let first = &locks[0];
                println!(
                    "First lock: path={:?}, timeout={:?}",
                    first.path, first.timeout
                );
            } else {
                println!("No locks found");
            }
        }
        Err(e) => {
            println!("Lock listing failed: {:?}", e);
        }
    }
}
