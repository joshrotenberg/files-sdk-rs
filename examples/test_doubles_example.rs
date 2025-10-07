//! Test Doubles Example
//!
//! Demonstrates using hand-written test doubles (fakes) for testing.
//!
//! This approach is useful when you want to:
//! - Track state changes during testing (e.g., uploaded files)
//! - Avoid external dependencies like mockall
//! - Have full control over test behavior
//! - Test integration between multiple components
//!
//! Run tests with:
//! ```bash
//! cargo test --example test_doubles_example
//! ```

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Uploaded file record
#[derive(Debug, Clone, PartialEq)]
pub struct UploadedFile {
    pub path: String,
    pub data: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

/// Fake Files.com client for testing
///
/// Records all uploads and provides methods to inspect state
#[derive(Clone)]
pub struct FakeFilesClient {
    uploaded_files: Arc<Mutex<Vec<UploadedFile>>>,
    should_fail: Arc<Mutex<bool>>,
}

impl FakeFilesClient {
    pub fn new() -> Self {
        Self {
            uploaded_files: Arc::new(Mutex::new(Vec::new())),
            should_fail: Arc::new(Mutex::new(false)),
        }
    }

    /// Configure the fake to fail on next upload
    pub fn set_should_fail(&self, fail: bool) {
        *self.should_fail.lock().unwrap() = fail;
    }

    /// Upload a file (records it in memory)
    pub fn upload(&self, path: &str, data: &[u8]) -> Result<(), String> {
        if *self.should_fail.lock().unwrap() {
            return Err("Simulated upload failure".to_string());
        }

        let file = UploadedFile {
            path: path.to_string(),
            data: data.to_vec(),
            metadata: HashMap::new(),
        };

        self.uploaded_files.lock().unwrap().push(file);
        Ok(())
    }

    /// Upload a file with metadata
    pub fn upload_with_metadata(
        &self,
        path: &str,
        data: &[u8],
        metadata: HashMap<String, String>,
    ) -> Result<(), String> {
        if *self.should_fail.lock().unwrap() {
            return Err("Simulated upload failure".to_string());
        }

        let file = UploadedFile {
            path: path.to_string(),
            data: data.to_vec(),
            metadata,
        };

        self.uploaded_files.lock().unwrap().push(file);
        Ok(())
    }

    /// Get all uploaded files
    pub fn get_uploaded_files(&self) -> Vec<UploadedFile> {
        self.uploaded_files.lock().unwrap().clone()
    }

    /// Check if a file was uploaded
    pub fn was_uploaded(&self, path: &str) -> bool {
        self.uploaded_files
            .lock()
            .unwrap()
            .iter()
            .any(|f| f.path == path)
    }

    /// Get upload count
    pub fn upload_count(&self) -> usize {
        self.uploaded_files.lock().unwrap().len()
    }

    /// Clear all uploads
    pub fn clear(&self) {
        self.uploaded_files.lock().unwrap().clear();
    }
}

impl Default for FakeFilesClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Example service using FakeFilesClient
pub struct DocumentService {
    client: FakeFilesClient,
}

impl DocumentService {
    pub fn new(client: FakeFilesClient) -> Self {
        Self { client }
    }

    pub fn save_document(&self, name: &str, content: &str) -> Result<(), String> {
        let path = format!("/documents/{}", name);
        self.client.upload(&path, content.as_bytes())
    }

    pub fn save_versioned_document(
        &self,
        name: &str,
        content: &str,
        version: &str,
    ) -> Result<(), String> {
        let path = format!("/documents/{}", name);
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), version.to_string());
        self.client
            .upload_with_metadata(&path, content.as_bytes(), metadata)
    }

    pub fn batch_save(&self, documents: Vec<(&str, &str)>) -> Result<usize, String> {
        let mut count = 0;
        for (name, content) in documents {
            self.save_document(name, content)?;
            count += 1;
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_service_uploads_file() {
        // Arrange
        let fake_client = FakeFilesClient::new();
        let service = DocumentService::new(fake_client.clone());

        // Act
        let result = service.save_document("readme.txt", "Hello, World!");

        // Assert
        assert!(result.is_ok());
        assert_eq!(fake_client.upload_count(), 1);
        assert!(fake_client.was_uploaded("/documents/readme.txt"));

        let files = fake_client.get_uploaded_files();
        assert_eq!(files[0].path, "/documents/readme.txt");
        assert_eq!(files[0].data, b"Hello, World!");
    }

    #[test]
    fn test_versioned_document_includes_metadata() {
        // Arrange
        let fake_client = FakeFilesClient::new();
        let service = DocumentService::new(fake_client.clone());

        // Act
        let result =
            service.save_versioned_document("config.json", "{\"key\": \"value\"}", "1.0.0");

        // Assert
        assert!(result.is_ok());

        let files = fake_client.get_uploaded_files();
        assert_eq!(files[0].metadata.get("version"), Some(&"1.0.0".to_string()));
    }

    #[test]
    fn test_batch_save_tracks_all_uploads() {
        // Arrange
        let fake_client = FakeFilesClient::new();
        let service = DocumentService::new(fake_client.clone());

        let documents = vec![
            ("file1.txt", "content1"),
            ("file2.txt", "content2"),
            ("file3.txt", "content3"),
        ];

        // Act
        let result = service.batch_save(documents);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);
        assert_eq!(fake_client.upload_count(), 3);
        assert!(fake_client.was_uploaded("/documents/file1.txt"));
        assert!(fake_client.was_uploaded("/documents/file2.txt"));
        assert!(fake_client.was_uploaded("/documents/file3.txt"));
    }

    #[test]
    fn test_handles_upload_failure() {
        // Arrange
        let fake_client = FakeFilesClient::new();
        fake_client.set_should_fail(true);
        let service = DocumentService::new(fake_client.clone());

        // Act
        let result = service.save_document("fail.txt", "should fail");

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Simulated upload failure");
        assert_eq!(fake_client.upload_count(), 0);
    }

    #[test]
    fn test_batch_save_stops_on_first_error() {
        // Arrange
        let fake_client = FakeFilesClient::new();
        let service = DocumentService::new(fake_client.clone());

        // Upload first file successfully
        service.save_document("file1.txt", "content1").unwrap();

        // Now make it fail
        fake_client.set_should_fail(true);

        let documents = vec![("file2.txt", "content2"), ("file3.txt", "content3")];

        // Act
        let result = service.batch_save(documents);

        // Assert
        assert!(result.is_err());
        assert_eq!(fake_client.upload_count(), 1); // Only the first upload before batch
    }
}

fn main() {
    println!("Test Doubles Example");
    println!("====================\n");
    println!("This example demonstrates hand-written test doubles (fakes).");
    println!("\nRun the tests with:");
    println!("  cargo test --example test_doubles_example");
    println!("\nKey concepts:");
    println!("- Create fake implementations that track state");
    println!("- Use Arc<Mutex<Vec<T>>> to record operations");
    println!("- Provide inspection methods to verify behavior");
    println!("- No external mocking libraries required");
}
