use files_sdk::{FilesClient, sharing::BundleHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("FILES_API_KEY")?;
    let client = FilesClient::builder().api_key(&api_key).build()?;
    let handler = BundleHandler::new(client);

    // Create share link for files
    let bundle = handler
        .create(
            vec!["/reports/Q4.pdf".to_string()],
            None,               // password
            Some("2024-12-31"), // expires_at
            None,               // max_uses
            None,               // description
            None,               // note
            None,               // code
            None,               // require_registration
            None,               // permissions
        )
        .await?;

    println!("Created share link:");
    println!("  URL: {}", bundle.url.as_deref().unwrap_or("N/A"));
    println!("  Code: {}", bundle.code.as_deref().unwrap_or("N/A"));
    if let Some(expires_at) = bundle.expires_at {
        println!("  Expires: {}", expires_at);
    }

    // Create password-protected share link
    let protected_bundle = handler
        .create(
            vec!["/sensitive/data.xlsx".to_string()],
            Some("secure123"),  // password
            Some("2024-10-10"), // expires_at
            None,               // max_uses
            None,               // description
            None,               // note
            None,               // code
            None,               // require_registration
            None,               // permissions
        )
        .await?;

    println!("\nCreated password-protected share link:");
    println!(
        "  URL: {}",
        protected_bundle.url.as_deref().unwrap_or("N/A")
    );
    println!("  Password: secure123");

    // List all bundles
    let (bundles, _pagination) = handler.list(None, None, Some(100)).await?;
    println!("\nTotal active share links: {}", bundles.len());
    for bundle in bundles {
        println!(
            "  {} - {}",
            bundle.code.as_deref().unwrap_or("N/A"),
            bundle.url.as_deref().unwrap_or("N/A")
        );
    }

    // Delete a bundle (cleanup)
    // if let Some(bundle_id) = bundle.id {
    //     handler.delete(bundle_id).await?;
    //     println!("\nDeleted share link: {}", bundle.code.as_deref().unwrap_or("N/A"));
    // }

    Ok(())
}
