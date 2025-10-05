use files_sdk::{FilesClient, files::FileHandler, files::FolderHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("FILES_API_KEY")?;
    let client = FilesClient::builder().api_key(&api_key).build()?;

    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client);

    // Create folder with automatic parent directory creation
    folder_handler.create_folder("/reports/2024", true).await?;
    println!("Created folder: /reports/2024");

    // Upload a file
    let data = b"Report data for Q4 2024";
    file_handler
        .upload_file("/reports/2024/summary.pdf", data)
        .await?;
    println!("Uploaded: /reports/2024/summary.pdf");

    // Download a file
    let file_entity = file_handler
        .download_file("/reports/2024/summary.pdf")
        .await?;
    println!(
        "Downloaded: {} ({} bytes)",
        file_entity.path.as_deref().unwrap_or("N/A"),
        file_entity.size.unwrap_or(0)
    );

    // Copy file
    file_handler
        .copy_file("/reports/2024/summary.pdf", "/backups/summary.pdf")
        .await?;
    println!("Copied to: /backups/summary.pdf");

    // Move file
    file_handler
        .move_file("/backups/summary.pdf", "/archive/summary.pdf")
        .await?;
    println!("Moved to: /archive/summary.pdf");

    // List folder contents
    let (files, _pagination) = folder_handler
        .list_folder("/reports", Some(100), None)
        .await?;
    println!("\nFiles in /reports:");
    for file in files {
        println!(
            "  {}: {} bytes",
            file.path.as_deref().unwrap_or("N/A"),
            file.size.unwrap_or(0)
        );
    }

    // Delete file
    file_handler
        .delete_file("/archive/summary.pdf", false)
        .await?;
    println!("\nDeleted: /archive/summary.pdf");

    Ok(())
}
