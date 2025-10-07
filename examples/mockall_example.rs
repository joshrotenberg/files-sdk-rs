//! Mockall Testing Example
//!
//! Demonstrates using mockall to create trait-based mocks for testing.
//!
//! This approach is useful when you want to:
//! - Test code that depends on the Files.com SDK without hitting the API
//! - Verify specific methods are called with expected arguments
//! - Control return values and test error paths
//!
//! Run tests with:
//! ```bash
//! cargo test --example mockall_example
//! ```

#[cfg(test)]
use mockall::automock;

/// Trait representing file upload operations
///
/// By using a trait, we can create mock implementations for testing
/// while the real implementation uses the actual Files.com SDK.
#[cfg_attr(test, automock)]
pub trait FileUploader {
    fn upload(&self, path: &str, data: &[u8]) -> Result<(), String>;
    fn upload_with_metadata(
        &self,
        path: &str,
        data: &[u8],
        metadata: std::collections::HashMap<String, String>,
    ) -> Result<(), String>;
}

/// Real implementation using the Files.com SDK
pub struct RealFileUploader {
    _client: files_sdk::FilesClient,
}

impl RealFileUploader {
    pub fn new(client: files_sdk::FilesClient) -> Self {
        Self { _client: client }
    }
}

impl FileUploader for RealFileUploader {
    fn upload(&self, _path: &str, _data: &[u8]) -> Result<(), String> {
        // In a real implementation, this would use:
        // let file_handler = FileHandler::new(self._client.clone());
        // file_handler.upload_file(_path, _data).await.map_err(|e| e.to_string())
        Ok(())
    }

    fn upload_with_metadata(
        &self,
        path: &str,
        data: &[u8],
        _metadata: std::collections::HashMap<String, String>,
    ) -> Result<(), String> {
        self.upload(path, data)
    }
}

/// Example service that depends on FileUploader
pub struct BackupService<U: FileUploader> {
    uploader: U,
}

impl<U: FileUploader> BackupService<U> {
    pub fn new(uploader: U) -> Self {
        Self { uploader }
    }

    pub fn backup_file(&self, local_path: &str, remote_path: &str) -> Result<(), String> {
        let data = std::fs::read(local_path).map_err(|e| e.to_string())?;
        self.uploader.upload(remote_path, &data)
    }

    pub fn backup_with_tags(
        &self,
        local_path: &str,
        remote_path: &str,
        tags: std::collections::HashMap<String, String>,
    ) -> Result<(), String> {
        let data = std::fs::read(local_path).map_err(|e| e.to_string())?;
        self.uploader.upload_with_metadata(remote_path, &data, tags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use std::collections::HashMap;

    #[test]
    fn test_backup_service_calls_upload() {
        // Arrange: Create a mock uploader
        let mut mock_uploader = MockFileUploader::new();

        // Expect upload to be called once with specific arguments
        mock_uploader
            .expect_upload()
            .with(eq("/backup/file.txt"), eq(b"test data".as_slice()))
            .times(1)
            .returning(|_, _| Ok(()));

        let service = BackupService::new(mock_uploader);

        // Act: Create a temporary file and backup
        let temp_file = std::env::temp_dir().join("test_file.txt");
        std::fs::write(&temp_file, b"test data").unwrap();

        let result = service.backup_file(temp_file.to_str().unwrap(), "/backup/file.txt");

        // Assert
        assert!(result.is_ok());

        // Cleanup
        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_backup_service_handles_upload_failure() {
        // Arrange: Mock that returns an error
        let mut mock_uploader = MockFileUploader::new();

        mock_uploader
            .expect_upload()
            .times(1)
            .returning(|_, _| Err("Upload failed".to_string()));

        let service = BackupService::new(mock_uploader);

        // Act
        let temp_file = std::env::temp_dir().join("test_file2.txt");
        std::fs::write(&temp_file, b"test data").unwrap();

        let result = service.backup_file(temp_file.to_str().unwrap(), "/backup/file.txt");

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Upload failed");

        // Cleanup
        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_backup_with_metadata() {
        // Arrange
        let mut mock_uploader = MockFileUploader::new();

        let expected_metadata = {
            let mut map = HashMap::new();
            map.insert("env".to_string(), "production".to_string());
            map.insert("version".to_string(), "1.0".to_string());
            map
        };

        mock_uploader
            .expect_upload_with_metadata()
            .with(
                eq("/backup/config.json"),
                eq(b"config data".as_slice()),
                eq(expected_metadata.clone()),
            )
            .times(1)
            .returning(|_, _, _| Ok(()));

        let service = BackupService::new(mock_uploader);

        // Act
        let temp_file = std::env::temp_dir().join("test_config.json");
        std::fs::write(&temp_file, b"config data").unwrap();

        let result = service.backup_with_tags(
            temp_file.to_str().unwrap(),
            "/backup/config.json",
            expected_metadata,
        );

        // Assert
        assert!(result.is_ok());

        // Cleanup
        std::fs::remove_file(temp_file).ok();
    }
}

fn main() {
    println!("Mockall Testing Example");
    println!("=======================\n");
    println!("This example demonstrates trait-based mocking with mockall.");
    println!("\nRun the tests with:");
    println!("  cargo test --example mockall_example");
    println!("\nKey concepts:");
    println!("- Define a trait for operations you want to mock");
    println!("- Use #[cfg_attr(test, automock)] to generate mocks");
    println!("- Set expectations on method calls and return values");
    println!("- Verify behavior without hitting real APIs");
}
