//! Real API integration tests for ChildSiteManagementPolicyHandler

use crate::real::*;
use files_sdk::ChildSiteManagementPolicyHandler;

#[tokio::test]
async fn test_real_api_list_child_site_management_policies() {
    let client = get_test_client();
    let handler = ChildSiteManagementPolicyHandler::new(client);

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((policies, pagination)) => {
            println!("Listed {} child site management policies", policies.len());
            println!("Pagination: {:?}", pagination);
            if let Some(first) = policies.first() {
                println!("Sample policy: {:?}", first);
            }
        }
        Err(e) => {
            println!("List failed (may require enterprise/MSP plan): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_get_child_site_management_policy() {
    let client = get_test_client();
    let handler = ChildSiteManagementPolicyHandler::new(client);

    let list_result = handler.list(None, Some(1)).await;

    match list_result {
        Ok((policies, _)) =>
        {
            #[allow(clippy::collapsible_if)]
            if let Some(policy) = policies.first() {
                if let Some(id) = policy.data.get("id").and_then(|v| v.as_i64()) {
                    let get_result = handler.get(id).await;
                    match get_result {
                        Ok(retrieved) => {
                            println!("Retrieved policy: {:?}", retrieved);
                        }
                        Err(e) => {
                            println!("Failed to get policy: {:?}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Could not list policies: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_child_site_management_policies_pagination() {
    let client = get_test_client();
    let handler = ChildSiteManagementPolicyHandler::new(client);

    let result = handler.list(None, Some(1)).await;

    match result {
        Ok((policies, pagination)) => {
            println!("First page: {} policies", policies.len());

            if let Some(cursor) = pagination.cursor_next {
                let result2 = handler.list(Some(cursor), Some(1)).await;
                match result2 {
                    Ok((policies2, _)) => {
                        println!("Second page: {} policies", policies2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Policies not available: {:?}", e);
        }
    }
}
