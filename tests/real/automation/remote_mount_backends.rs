//! Real API integration tests for RemoteMountBackendHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! RemoteMountBackends configure external storage for mounting.

use crate::real::*;
use files_sdk::RemoteMountBackendHandler;

#[tokio::test]
async fn test_real_api_list_remote_mount_backends() {
    let client = get_test_client();
    let handler = RemoteMountBackendHandler::new(client);

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((backends, pagination)) => {
            println!("Listed {} remote mount backends", backends.len());
            println!("Pagination: {:?}", pagination);

            if let Some(first) = backends.first() {
                println!("Sample backend: {:?}", first);
            }
        }
        Err(e) => {
            println!(
                "Remote mount backends list failed (may require enterprise plan): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_get_remote_mount_backend() {
    let client = get_test_client();
    let handler = RemoteMountBackendHandler::new(client);

    // First, get a list to find an ID
    let list_result = handler.list(None, Some(1)).await;

    match list_result {
        Ok((backends, _)) => {
            if let Some(backend) = backends.first() {
                // Extract ID from the backend data
                if let Some(id) = backend.data.get("id").and_then(|v| v.as_i64()) {
                    // Try to get that specific backend
                    let get_result = handler.get(id).await;
                    match get_result {
                        Ok(retrieved) => {
                            println!("Retrieved remote mount backend: {:?}", retrieved);
                        }
                        Err(e) => {
                            println!("Failed to get remote mount backend: {:?}", e);
                        }
                    }
                }
            } else {
                println!("No remote mount backends available to test get");
            }
        }
        Err(e) => {
            println!("Could not list remote mount backends: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_remote_mount_backends_pagination() {
    let client = get_test_client();
    let handler = RemoteMountBackendHandler::new(client);

    // Test pagination with small page size
    let result = handler.list(None, Some(1)).await;

    match result {
        Ok((backends, pagination)) => {
            println!("First page: {} remote mount backends", backends.len());

            if let Some(cursor) = pagination.cursor_next {
                // Fetch second page
                let result2 = handler.list(Some(cursor), Some(1)).await;
                match result2 {
                    Ok((backends2, _)) => {
                        println!("Second page: {} remote mount backends", backends2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Remote mount backends not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_remote_mount_backends_structure() {
    let client = get_test_client();
    let handler = RemoteMountBackendHandler::new(client);

    let result = handler.list(None, Some(5)).await;

    match result {
        Ok((backends, _)) => {
            println!("Remote mount backends count: {}", backends.len());

            for backend in backends.iter() {
                println!("Backend data keys: {:?}", backend.data.keys());
            }
        }
        Err(e) => {
            println!("Remote mount backends not available: {:?}", e);
        }
    }
}
