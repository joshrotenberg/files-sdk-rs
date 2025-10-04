//! Common utilities for real API integration tests
//!
//! These utilities help with running tests against the actual Files.com API,
//! including client setup, cleanup helpers, and test environment validation.

use files_sdk::FilesClient;

/// Gets a test client configured with the API key from environment variables
///
/// # Panics
///
/// Panics if the FILES_API_KEY environment variable is not set
pub fn get_test_client() -> FilesClient {
    let api_key = std::env::var("FILES_API_KEY")
        .expect("FILES_API_KEY environment variable must be set for integration tests");
    FilesClient::builder().api_key(&api_key).build().unwrap()
}

/// Standard test folder path for integration tests
pub const TEST_FOLDER: &str = "/integration-tests";

/// Generates a unique test file path with timestamp
pub fn unique_test_path(base_name: &str) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("{}/{}-{}", TEST_FOLDER, base_name, timestamp)
}

/// Helper to ensure the test folder exists
pub async fn ensure_test_folder(client: &FilesClient) {
    use files_sdk::FolderHandler;
    let folder_handler = FolderHandler::new(client.clone());
    let _ = folder_handler.create_folder(TEST_FOLDER, true).await;
}

/// Helper to clean up a test file, ignoring errors
pub async fn cleanup_file(client: &FilesClient, path: &str) {
    use files_sdk::FileHandler;
    let file_handler = FileHandler::new(client.clone());
    let _ = file_handler.delete_file(path, false).await;
}

/// Helper to clean up a test folder, ignoring errors
pub async fn cleanup_folder(client: &FilesClient, path: &str) {
    use files_sdk::FolderHandler;
    let folder_handler = FolderHandler::new(client.clone());
    let _ = folder_handler.delete_folder(path, true).await;
}
