use files_sdk::{FilesClient, users::UserHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("FILES_API_KEY")?;
    let client = FilesClient::builder().api_key(&api_key).build()?;
    let handler = UserHandler::new(client);

    // List users with pagination
    let (users, pagination) = handler.list(None, Some(50)).await?;
    println!("Found {} users", users.len());
    for user in &users {
        println!(
            "  {} ({})",
            user.username.as_deref().unwrap_or("N/A"),
            user.email.as_deref().unwrap_or("")
        );
    }

    if let Some(next_cursor) = pagination.cursor_next {
        println!("\nNext page cursor: {}", next_cursor);
    }

    // Get specific user (example with first user if available)
    if let Some(first_user) = users.first()
        && let Some(user_id) = first_user.id
    {
        let user = handler.get(user_id).await?;
        println!(
            "\nUser details for {}:",
            user.username.as_deref().unwrap_or("N/A")
        );
        println!("  ID: {}", user.id.unwrap_or(0));
        println!("  Email: {}", user.email.as_deref().unwrap_or("N/A"));
    }

    // Create user (commented out - requires admin permissions)
    // let new_user = handler
    //     .create("newuser@example.com", "newuser", None)
    //     .await?;
    // println!("\nCreated user: {} (ID: {})", new_user.username, new_user.id);

    // Update user (commented out - requires admin permissions and valid user ID)
    // handler
    //     .update(123, Some("updated@example.com"), None)
    //     .await?;
    // println!("Updated user 123");

    Ok(())
}
