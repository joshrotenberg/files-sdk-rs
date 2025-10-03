use files_sdk::{FilesClient, Result, UserHandler};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = "faa9da7771d67901eb18f9067323537f31b2ac0a16179c3f7637f2c8448381ba";

    let client = FilesClient::builder().api_key(api_key).build()?;

    let user_handler = UserHandler::new(client);

    // Test 1: List users
    println!("=== Test 1: List Users ===");
    match user_handler.list(None, Some(10)).await {
        Ok((users, pagination)) => {
            println!("Found {} users", users.len());
            for user in users.iter().take(3) {
                println!(
                    "  - {} ({}): {}",
                    user.username.as_deref().unwrap_or("no username"),
                    user.id.unwrap_or(0),
                    user.email.as_deref().unwrap_or("no email")
                );
            }
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );
        }
        Err(e) => println!("Error listing users: {:?}", e),
    }

    println!("\n=== Test 2: Get Current User (ID 1) ===");
    match user_handler.get(1).await {
        Ok(user) => {
            println!("User details:");
            println!("  Username: {:?}", user.username);
            println!("  Email: {:?}", user.email);
            println!("  Name: {:?}", user.name);
            println!("  Site Admin: {:?}", user.site_admin);
            println!("  Created: {:?}", user.created_at);
        }
        Err(e) => println!("Error getting user: {:?}", e),
    }

    println!("\n=== Spot Check Complete ===");
    Ok(())
}
