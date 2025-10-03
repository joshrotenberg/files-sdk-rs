use files_sdk::{FileHandler, FilesClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let api_key =
        std::env::var("FILES_API_KEY").expect("FILES_API_KEY environment variable not set");

    let client = FilesClient::builder().api_key(&api_key).build()?;

    let file_handler = FileHandler::new(client);

    let test_files = vec![
        "/test1-small.txt",
        "/test2-larger.txt",
        "/test3-binary.dat",
        "/subdir/test4-nested.txt",
    ];

    println!("Verifying uploaded files exist...\n");

    for path in test_files {
        match file_handler.download_file(path).await {
            Ok(file) => {
                println!("✓ {} exists:", path);
                println!("  Display name: {:?}", file.display_name);
                println!("  Size: {:?} bytes", file.size);
                println!("  Type: {:?}", file.file_type);
                println!();
            }
            Err(e) => {
                println!("✗ {} - Error: {:?}\n", path, e);
            }
        }
    }

    Ok(())
}
