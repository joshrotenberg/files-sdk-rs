use files_sdk::{FilesClient, GroupHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "faa9da7771d67901eb18f9067323537f31b2ac0a16179c3f7637f2c8448381ba";

    let client = FilesClient::builder().api_key(api_key).build()?;

    let handler = GroupHandler::new(client);

    // Test list groups
    println!("Testing list groups...");
    match handler.list(None, None).await {
        Ok((groups, pagination)) => {
            println!("Found {} groups", groups.len());
            for group in &groups {
                println!("  Group: {:?}", group.name);
                println!("    ID: {:?}", group.id);
                println!("    Admin IDs: {:?}", group.admin_ids);
                println!("    User IDs: {:?}", group.user_ids);
            }
            println!("Pagination: {:?}", pagination);

            // If we have groups, test get on the first one
            if let Some(first_group) = groups.first()
                && let Some(id) = first_group.id
            {
                println!("\nTesting get group {}...", id);
                match handler.get(id).await {
                    Ok(group) => {
                        println!("Got group: {:?}", group.name);
                    }
                    Err(e) => {
                        println!("Error getting group: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error listing groups: {}", e);
        }
    }

    Ok(())
}
