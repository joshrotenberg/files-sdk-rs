//! Real API integration tests for ProjectHandler

use crate::real::*;
use files_sdk::ProjectHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_projects() {
    let client = get_test_client();
    let handler = ProjectHandler::new(client);

    println!("Testing project listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((projects, pagination)) => {
            println!("Successfully listed {} projects", projects.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !projects.is_empty() {
                let first = &projects[0];
                println!(
                    "First project: id={:?}, global_access={:?}",
                    first.id, first.global_access
                );
            } else {
                println!("No projects found");
            }
        }
        Err(e) => {
            println!("Project listing failed: {:?}", e);
        }
    }
}
