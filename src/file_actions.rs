//! File action operations
//!
//! This module provides specialized file operations including:
//! - Begin upload (first stage of file upload process)
//! - Copy files
//! - Move files
//! - Get metadata
//!
//! The most important operation here is `begin_upload`, which must be called
//! before uploading any file to Files.com.

use crate::{FileUploadPartEntity, FilesClient, Result};
use serde_json::json;

/// Handler for file action operations
///
/// Provides methods for specialized file operations that are separate
/// from basic file CRUD operations.
#[derive(Debug, Clone)]
pub struct FileActionHandler {
    client: FilesClient,
}

impl FileActionHandler {
    /// Creates a new FileActionHandler
    ///
    /// # Arguments
    ///
    /// * `client` - FilesClient instance
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// Begin file upload (Stage 1 of upload process)
    ///
    /// This must be called before uploading any file. It returns upload URLs
    /// and parameters needed to perform the actual upload.
    ///
    /// # Arguments
    ///
    /// * `path` - Destination path for the file
    /// * `size` - Total size of file in bytes (optional)
    /// * `mkdir_parents` - Create parent directories if they don't exist
    ///
    /// # Returns
    ///
    /// Returns a vector of `FileUploadPartEntity` containing upload URLs and parameters.
    /// For small files, this will typically be a single element. For large files,
    /// it may contain multiple parts for parallel upload.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use files_sdk::{FilesClient, FileActionHandler};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder()
    ///     .api_key("your-api-key")
    ///     .build()?;
    ///
    /// let handler = FileActionHandler::new(client);
    ///
    /// // Begin upload for a 1KB file
    /// let upload_info = handler
    ///     .begin_upload("/uploads/test.txt", Some(1024), true)
    ///     .await?;
    ///
    /// println!("Upload URL: {:?}", upload_info[0].upload_uri);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn begin_upload(
        &self,
        path: &str,
        size: Option<i64>,
        mkdir_parents: bool,
    ) -> Result<Vec<FileUploadPartEntity>> {
        let mut body = json!({
            "mkdir_parents": mkdir_parents,
        });

        if let Some(size) = size {
            body["size"] = json!(size);
        }

        let endpoint = format!("/file_actions/begin_upload{}", path);
        let response = self.client.post_raw(&endpoint, body).await?;

        // Response can be a single object or an array
        if response.is_array() {
            Ok(serde_json::from_value(response)?)
        } else {
            // Single object - wrap in array
            Ok(vec![serde_json::from_value(response)?])
        }
    }

    /// Begin multi-part upload for large files
    ///
    /// For files larger than the default part size, this allows requesting
    /// multiple upload parts for parallel uploading.
    ///
    /// # Arguments
    ///
    /// * `path` - Destination path for the file
    /// * `size` - Total size of file in bytes
    /// * `parts` - Number of parts to split the upload into
    /// * `mkdir_parents` - Create parent directories if they don't exist
    ///
    /// # Returns
    ///
    /// Returns a vector of `FileUploadPartEntity`, one for each part
    pub async fn begin_multipart_upload(
        &self,
        path: &str,
        size: i64,
        parts: i32,
        mkdir_parents: bool,
    ) -> Result<Vec<FileUploadPartEntity>> {
        let body = json!({
            "size": size,
            "parts": parts,
            "mkdir_parents": mkdir_parents,
        });

        let endpoint = format!("/file_actions/begin_upload{}", path);
        let response = self.client.post_raw(&endpoint, body).await?;

        // Response should be an array for multipart
        if response.is_array() {
            Ok(serde_json::from_value(response)?)
        } else {
            Ok(vec![serde_json::from_value(response)?])
        }
    }

    /// Copy a file to a new location
    ///
    /// # Arguments
    ///
    /// * `path` - Source file path
    /// * `destination` - Destination path
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FileActionHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FileActionHandler::new(client);
    /// handler.copy_file("/source/file.txt", "/dest/file.txt").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn copy_file(&self, path: &str, destination: &str) -> Result<()> {
        let body = json!({
            "destination": destination,
        });

        let endpoint = format!("/file_actions/copy{}", path);
        self.client.post_raw(&endpoint, body).await?;
        Ok(())
    }

    /// Move a file to a new location
    ///
    /// # Arguments
    ///
    /// * `path` - Source file path
    /// * `destination` - Destination path
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FileActionHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FileActionHandler::new(client);
    /// handler.move_file("/source/file.txt", "/dest/file.txt").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn move_file(&self, path: &str, destination: &str) -> Result<()> {
        let body = json!({
            "destination": destination,
        });

        let endpoint = format!("/file_actions/move{}", path);
        self.client.post_raw(&endpoint, body).await?;
        Ok(())
    }

    /// Get file metadata without downloading
    ///
    /// This is useful when you want file information without generating
    /// download URLs or logging download activity.
    ///
    /// # Arguments
    ///
    /// * `path` - File path
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FileActionHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FileActionHandler::new(client);
    /// let metadata = handler.get_metadata("/path/to/file.txt").await?;
    /// println!("Size: {:?}", metadata.size);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_metadata(&self, path: &str) -> Result<crate::FileEntity> {
        let endpoint = format!("/file_actions/metadata{}", path);
        let response = self.client.post_raw(&endpoint, json!({})).await?;
        Ok(serde_json::from_value(response)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = FileActionHandler::new(client);
    }
}
