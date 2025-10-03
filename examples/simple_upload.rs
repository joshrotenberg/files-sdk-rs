use files_sdk::{FilesClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key =
        std::env::var("FILES_API_KEY").expect("FILES_API_KEY environment variable not set");

    let client = FilesClient::builder().api_key(&api_key).build()?;

    // Create test data
    let test_data = b"Hello from files-sdk Rust! This is a test upload.";
    let test_path = "/test-upload.txt";

    println!("Uploading {} bytes to {}", test_data.len(), test_path);

    // Try the upload
    let file_handler = files_sdk::FileHandler::new(client);
    let result = file_handler.upload_file(test_path, test_data).await?;

    println!("Upload successful!");
    println!("File: {:?}", result.path);
    println!("Size: {:?} bytes", result.size);

    Ok(())
}
