use files_sdk::{FileHandler, FilesClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key =
        std::env::var("FILES_API_KEY").expect("FILES_API_KEY environment variable not set");

    let client = FilesClient::builder().api_key(&api_key).build()?;

    let file_handler = FileHandler::new(client);

    // Test 1: Small text file
    println!("Test 1: Small text file");
    let result1 = file_handler
        .upload_file("/test1-small.txt", b"Hello from Rust SDK!")
        .await?;
    println!("✓ Uploaded: {:?}, size: {:?}", result1.path, result1.size);

    // Test 2: Larger text file
    println!("\nTest 2: Larger text file");
    let large_text = "This is a larger test file.\n".repeat(100);
    let result2 = file_handler
        .upload_file("/test2-larger.txt", large_text.as_bytes())
        .await?;
    println!("✓ Uploaded: {:?}, size: {:?}", result2.path, result2.size);

    // Test 3: Binary-ish data
    println!("\nTest 3: Binary data");
    let binary_data: Vec<u8> = (0..=255).cycle().take(1000).collect();
    let result3 = file_handler
        .upload_file("/test3-binary.dat", &binary_data)
        .await?;
    println!("✓ Uploaded: {:?}, size: {:?}", result3.path, result3.size);

    // Test 4: File in subdirectory (with mkdir_parents)
    println!("\nTest 4: File in subdirectory");
    let result4 = file_handler
        .upload_file("/subdir/test4-nested.txt", b"File in subdirectory")
        .await?;
    println!("✓ Uploaded: {:?}, size: {:?}", result4.path, result4.size);

    println!("\n✅ All 4 uploads successful!");

    Ok(())
}
