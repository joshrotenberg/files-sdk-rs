use files_sdk::{FilesClient, PermissionHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("FILES_API_KEY").expect("FILES_API_KEY environment variable not set");

    let client = FilesClient::builder().api_key(&api_key).build()?;

    let handler = PermissionHandler::new(client);

    // Test list permissions
    println!("Testing list permissions...");
    match handler.list(None, None).await {
        Ok((permissions, pagination)) => {
            println!("Found {} permissions", permissions.len());
            for permission in &permissions {
                println!("  Permission ID: {:?}", permission.id);
                println!("    Path: {:?}", permission.path);
                println!(
                    "    User: {:?} (ID: {:?})",
                    permission.username, permission.user_id
                );
                println!(
                    "    Group: {:?} (ID: {:?})",
                    permission.group_name, permission.group_id
                );
                println!("    Type: {:?}", permission.permission);
                println!("    Recursive: {:?}", permission.recursive);
            }
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!("Error listing permissions: {}", e);
        }
    }

    Ok(())
}
