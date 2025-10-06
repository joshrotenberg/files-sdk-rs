use files_sdk::{FilesClient, FilesError, files::FileHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("FILES_API_KEY")?;
    let client = FilesClient::builder().api_key(&api_key).build()?;
    let handler = FileHandler::new(client);

    // Example 1: Handle NotFound error
    match handler.download_file("/missing.txt").await {
        Ok(file) => println!("Downloaded: {}", file.path.as_deref().unwrap_or("N/A")),
        Err(FilesError::NotFound { message, .. }) => {
            eprintln!("File not found: {}", message);
        }
        Err(e) => eprintln!("Unexpected error: {}", e),
    }

    // Example 2: Handle authentication errors
    let bad_client = FilesClient::builder().api_key("invalid-key").build()?;
    let bad_handler = FileHandler::new(bad_client);

    match bad_handler.download_file("/test.txt").await {
        Ok(_) => println!("Success"),
        Err(FilesError::AuthenticationFailed { message, .. }) => {
            eprintln!("Auth failed: {}", message);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 3: Handle rate limiting
    match handler.download_file("/some/file.txt").await {
        Ok(file) => println!("Downloaded: {}", file.path.as_deref().unwrap_or("N/A")),
        Err(FilesError::RateLimited { message, .. }) => {
            eprintln!("Rate limited: {}", message);
            eprintln!("Consider implementing retry logic with exponential backoff");
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 4: Match on all error types
    let result = handler.upload_file("/test.txt", b"data").await;
    match result {
        Ok(file) => println!("Uploaded: {}", file.path.as_deref().unwrap_or("N/A")),
        Err(FilesError::BadRequest { message, .. }) => {
            eprintln!("Bad request (400): {}", message)
        }
        Err(FilesError::AuthenticationFailed { message, .. }) => {
            eprintln!("Auth failed (401): {}", message)
        }
        Err(FilesError::Forbidden { message, .. }) => {
            eprintln!("Forbidden (403): {}", message)
        }
        Err(FilesError::NotFound { message, .. }) => {
            eprintln!("Not found (404): {}", message)
        }
        Err(FilesError::Conflict { message, .. }) => {
            eprintln!("Conflict (409): {}", message)
        }
        Err(FilesError::PreconditionFailed { message, .. }) => {
            eprintln!("Precondition failed (412): {}", message)
        }
        Err(FilesError::UnprocessableEntity { message, .. }) => {
            eprintln!("Unprocessable (422): {}", message)
        }
        Err(FilesError::Locked { message, .. }) => {
            eprintln!("Locked (423): {}", message)
        }
        Err(FilesError::RateLimited { message, .. }) => {
            eprintln!("Rate limited (429): {}", message)
        }
        Err(FilesError::InternalServerError { message, .. }) => {
            eprintln!("Server error (500): {}", message)
        }
        Err(FilesError::ServiceUnavailable { message, .. }) => {
            eprintln!("Service unavailable (503): {}", message)
        }
        Err(FilesError::Request(e)) => {
            eprintln!("HTTP request error: {}", e)
        }
        Err(FilesError::JsonError(e)) => {
            eprintln!("JSON parsing error: {}", e)
        }
        Err(e) => {
            eprintln!("Other error: {}", e)
        }
    }

    Ok(())
}
