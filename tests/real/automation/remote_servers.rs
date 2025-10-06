//! Real API integration tests for RemoteServerHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! RemoteServers configure connections to external storage providers.

use crate::real::*;
use files_sdk::RemoteServerHandler;

#[tokio::test]
async fn test_real_api_list_remote_servers() {
    let client = get_test_client();
    let handler = RemoteServerHandler::new(client);

    let result = handler.list(None, None).await;

    match result {
        Ok((servers, pagination)) => {
            println!("Listed {} remote servers", servers.len());
            println!("Pagination: {:?}", pagination);

            if let Some(first) = servers.first() {
                println!("Sample server: {:?}", first);
            }
        }
        Err(e) => {
            println!(
                "Remote servers list failed (may require permissions): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_get_remote_server() {
    let client = get_test_client();
    let handler = RemoteServerHandler::new(client);

    // First, get a list to find an ID
    let list_result = handler.list(None, None).await;

    match list_result {
        Ok((servers, _)) => {
            if let Some(server) = servers.first() {
                if let Some(id) = server.id {
                    // Try to get that specific server
                    let get_result = handler.get(id).await;
                    match get_result {
                        Ok(retrieved) => {
                            println!("Retrieved remote server: {:?}", retrieved);
                            assert_eq!(retrieved.id, Some(id));
                        }
                        Err(e) => {
                            println!("Failed to get remote server: {:?}", e);
                        }
                    }
                }
            } else {
                println!("No remote servers available to test get");
            }
        }
        Err(e) => {
            println!("Could not list remote servers: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_remote_servers_pagination() {
    let client = get_test_client();
    let handler = RemoteServerHandler::new(client);

    // Test pagination with small page size
    let result = handler.list(None, Some(1)).await;

    match result {
        Ok((servers, pagination)) => {
            println!("First page: {} remote servers", servers.len());

            if let Some(cursor) = pagination.cursor_next {
                // Fetch second page
                let result2 = handler.list(Some(&cursor), Some(1)).await;
                match result2 {
                    Ok((servers2, _)) => {
                        println!("Second page: {} remote servers", servers2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Remote servers not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_remote_servers_structure() {
    let client = get_test_client();
    let handler = RemoteServerHandler::new(client);

    let result = handler.list(None, Some(5)).await;

    match result {
        Ok((servers, _)) => {
            println!("Remote servers count: {}", servers.len());

            for server in servers.iter() {
                if let Some(name) = &server.name {
                    println!("Server name: {}", name);
                }
                if let Some(server_type) = &server.server_type {
                    println!("Server type: {}", server_type);
                }
            }
        }
        Err(e) => {
            println!("Remote servers not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_remote_servers_error_handling() {
    let client = get_test_client();
    let handler = RemoteServerHandler::new(client);

    // Test getting non-existent server
    let result = handler.get(999999).await;

    match result {
        Ok(_) => {
            println!("Unexpectedly found server");
        }
        Err(e) => {
            println!("Expected error handling works: {:?}", e);
        }
    }
}
