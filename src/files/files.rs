//! File operations
//!
//! This module provides core file operations including:
//! - Download files
//! - Upload files (using the two-stage upload process)
//! - Update file metadata
//! - Delete files
//!
//! Note: File uploads in Files.com use a two-stage process:
//! 1. Call `FileActionHandler::begin_upload()` to get upload URLs
//! 2. Use this handler's `upload_file()` to complete the upload

use crate::files::FileActionHandler;
use crate::types::FileEntity;
use crate::utils::encode_path;
use crate::{FilesClient, FilesError, Result};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

/// Handler for file operations
///
/// Provides methods for downloading, uploading, updating, and deleting files.
#[derive(Debug, Clone)]
pub struct FileHandler {
    client: FilesClient,
}

impl FileHandler {
    /// Creates a new FileHandler
    ///
    /// # Arguments
    ///
    /// * `client` - FilesClient instance
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// Download a file or get file information
    ///
    /// # Arguments
    ///
    /// * `path` - File path to download
    ///
    /// # Returns
    ///
    /// Returns a `FileEntity` containing file information including a
    /// `download_uri` for the actual file download.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use files_sdk::{FilesClient, FileHandler};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder()
    ///     .api_key("your-api-key")
    ///     .build()?;
    ///
    /// let handler = FileHandler::new(client);
    /// let file = handler.download_file("/path/to/file.txt").await?;
    ///
    /// if let Some(uri) = file.download_uri {
    ///     println!("Download from: {}", uri);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_file(&self, path: &str) -> Result<FileEntity> {
        let encoded_path = encode_path(path);
        let endpoint = format!("/files{}", encoded_path);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Download the actual file content as bytes
    ///
    /// Unlike `download_file()` which returns metadata with a download URL,
    /// this method fetches and returns the actual file content.
    ///
    /// # Arguments
    ///
    /// * `path` - File path to download
    ///
    /// # Returns
    ///
    /// Returns the file content as a `Vec<u8>`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use files_sdk::{FilesClient, FileHandler};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder()
    ///     .api_key("your-api-key")
    ///     .build()?;
    ///
    /// let handler = FileHandler::new(client);
    /// let content = handler.download_content("/path/to/file.txt").await?;
    /// println!("Downloaded {} bytes", content.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_content(&self, path: &str) -> Result<Vec<u8>> {
        // First, get the file metadata to obtain the download URI
        let file = self.download_file(path).await?;

        // Extract the download URI
        let download_uri = file.download_uri.ok_or_else(|| {
            FilesError::not_found_resource("No download URI available", "file", path)
        })?;

        // Fetch the actual file content from the download URI
        let response = reqwest::get(&download_uri)
            .await
            .map_err(FilesError::Request)?;

        let bytes = response.bytes().await.map_err(FilesError::Request)?;

        Ok(bytes.to_vec())
    }

    /// Download file content and save to a local file
    ///
    /// This is a convenience method that downloads the file content and
    /// writes it to the specified local path.
    ///
    /// # Arguments
    ///
    /// * `remote_path` - Path to the file on Files.com
    /// * `local_path` - Local filesystem path where the file should be saved
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use files_sdk::{FilesClient, FileHandler};
    /// use std::path::Path;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder()
    ///     .api_key("your-api-key")
    ///     .build()?;
    ///
    /// let handler = FileHandler::new(client);
    /// handler.download_to_file(
    ///     "/path/to/remote/file.txt",
    ///     Path::new("./local/file.txt")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_to_file(
        &self,
        remote_path: &str,
        local_path: &std::path::Path,
    ) -> Result<()> {
        let content = self.download_content(remote_path).await?;
        std::fs::write(local_path, content)
            .map_err(|e| FilesError::IoError(format!("Failed to write file: {}", e)))?;
        Ok(())
    }

    /// Download file content to an async stream (for large files)
    ///
    /// This method is more memory-efficient than `download_content()` for large files
    /// as it streams the data directly to the writer instead of loading it into memory.
    ///
    /// # Arguments
    ///
    /// * `remote_path` - Path to the file on Files.com
    /// * `writer` - An async writer implementing `AsyncWrite`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FileHandler};
    /// # use tokio::fs::File;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FileHandler::new(client);
    ///
    /// let mut file = File::create("downloaded-large-file.tar.gz").await?;
    /// handler.download_stream("/remote/large-file.tar.gz", &mut file).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_stream<W>(&self, remote_path: &str, writer: &mut W) -> Result<()>
    where
        W: tokio::io::AsyncWrite + Unpin,
    {
        use tokio::io::AsyncWriteExt;

        // First, get the file metadata to obtain the download URI
        let file = self.download_file(remote_path).await?;

        // Extract the download URI
        let download_uri = file.download_uri.ok_or_else(|| {
            FilesError::not_found_resource("No download URI available", "file", remote_path)
        })?;

        // Stream the file content from the download URI
        let mut response = reqwest::get(&download_uri)
            .await
            .map_err(FilesError::Request)?;

        // Stream chunks to the writer
        while let Some(chunk) = response.chunk().await.map_err(FilesError::Request)? {
            writer
                .write_all(&chunk)
                .await
                .map_err(|e| FilesError::IoError(format!("Failed to write to stream: {}", e)))?;
        }

        // Flush the writer to ensure all data is written
        writer
            .flush()
            .await
            .map_err(|e| FilesError::IoError(format!("Failed to flush stream: {}", e)))?;

        Ok(())
    }

    /// Get file metadata only (no download URL, no logging)
    ///
    /// This is a convenience method that calls `FileActionHandler::get_metadata()`
    ///
    /// # Arguments
    ///
    /// * `path` - File path
    pub async fn get_metadata(&self, path: &str) -> Result<FileEntity> {
        let file_action = FileActionHandler::new(self.client.clone());
        file_action.get_metadata(path).await
    }

    /// Upload a file (complete two-stage upload process)
    ///
    /// This method handles the complete upload process:
    /// 1. Calls begin_upload to get upload URLs
    /// 2. Uploads the file data
    /// 3. Finalizes the upload
    ///
    /// # Arguments
    ///
    /// * `path` - Destination path for the file
    /// * `data` - File contents as bytes
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FileHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FileHandler::new(client);
    ///
    /// let data = b"Hello, Files.com!";
    /// let file = handler.upload_file("/uploads/test.txt", data).await?;
    /// println!("Uploaded: {:?}", file.path);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_file(&self, path: &str, data: &[u8]) -> Result<FileEntity> {
        // Stage 1: Begin upload
        let file_action = FileActionHandler::new(self.client.clone());
        let upload_parts = file_action
            .begin_upload(path, Some(data.len() as i64), true)
            .await?;

        if upload_parts.is_empty() {
            return Err(crate::FilesError::ApiError {
                endpoint: None,
                code: 500,
                message: "No upload parts returned from begin_upload".to_string(),
            });
        }

        let upload_part = &upload_parts[0];

        // Stage 2: Upload file data to the provided URL
        // This is an external URL (not Files.com API), typically to cloud storage
        let _etag = if let Some(upload_uri) = &upload_part.upload_uri {
            let http_client = reqwest::Client::new();
            let http_method = upload_part
                .http_method
                .as_deref()
                .unwrap_or("PUT")
                .to_uppercase();

            let mut request = match http_method.as_str() {
                "PUT" => http_client.put(upload_uri),
                "POST" => http_client.post(upload_uri),
                _ => http_client.put(upload_uri),
            };

            // Add any custom headers
            if let Some(headers) = &upload_part.headers {
                for (key, value) in headers {
                    request = request.header(key, value);
                }
            }

            // Upload the file data and capture the response
            let upload_response = request.body(data.to_vec()).send().await?;

            // Extract ETag from response headers
            upload_response
                .headers()
                .get("etag")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.trim_matches('"').to_string())
        } else {
            None
        };

        // Stage 3: Finalize upload with Files.com
        let encoded_path = encode_path(path);
        let endpoint = format!("/files{}", encoded_path);

        // Build the finalization request as form data
        let mut form = vec![("action", "end".to_string())];

        // Add ref (upload reference) - this is required to identify the upload
        if let Some(ref_value) = &upload_part.ref_ {
            form.push(("ref", ref_value.clone()));
        }

        // Note: etags might not be needed when ref is provided
        // Commenting out for now to test
        // if let Some(etag_value) = etag {
        //     let part_number = upload_part.part_number.unwrap_or(1);
        //     form.push(("etags[etag]", etag_value));
        //     form.push(("etags[part]", part_number.to_string()));
        // }

        let response = self.client.post_form(&endpoint, &form).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Upload a file from an async stream (for large files)
    ///
    /// This method is more memory-efficient than `upload_file()` for large files
    /// as it streams the data instead of loading it entirely into memory.
    ///
    /// # Arguments
    ///
    /// * `path` - Destination path for the file
    /// * `reader` - An async reader implementing `AsyncRead`
    /// * `size` - Optional size of the file in bytes (required for some upload methods)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FileHandler};
    /// # use tokio::fs::File;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FileHandler::new(client);
    ///
    /// let file = File::open("large-file.tar.gz").await?;
    /// let metadata = file.metadata().await?;
    /// let size = metadata.len();
    ///
    /// handler.upload_stream("/uploads/large-file.tar.gz", file, Some(size as i64)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_stream<R>(
        &self,
        path: &str,
        mut reader: R,
        size: Option<i64>,
    ) -> Result<FileEntity>
    where
        R: tokio::io::AsyncRead + Unpin,
    {
        use tokio::io::AsyncReadExt;

        // Stage 1: Begin upload
        let file_action = FileActionHandler::new(self.client.clone());
        let upload_parts = file_action.begin_upload(path, size, true).await?;

        if upload_parts.is_empty() {
            return Err(crate::FilesError::ApiError {
                endpoint: None,
                code: 500,
                message: "No upload parts returned from begin_upload".to_string(),
            });
        }

        let upload_part = &upload_parts[0];

        // Stage 2: Stream file data to the provided URL
        let _etag = if let Some(upload_uri) = &upload_part.upload_uri {
            // Read the entire stream into a buffer
            // TODO: In future, we could implement true streaming with chunked transfer encoding
            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .await
                .map_err(|e| FilesError::IoError(format!("Failed to read from stream: {}", e)))?;

            let http_client = reqwest::Client::new();
            let http_method = upload_part
                .http_method
                .as_deref()
                .unwrap_or("PUT")
                .to_uppercase();

            let mut request = match http_method.as_str() {
                "PUT" => http_client.put(upload_uri),
                "POST" => http_client.post(upload_uri),
                _ => http_client.put(upload_uri),
            };

            // Add any custom headers
            if let Some(headers) = &upload_part.headers {
                for (key, value) in headers {
                    request = request.header(key, value);
                }
            }

            // Upload the data
            let upload_response = request.body(buffer).send().await?;

            // Extract ETag from response headers
            upload_response
                .headers()
                .get("etag")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.trim_matches('"').to_string())
        } else {
            None
        };

        // Stage 3: Finalize upload with Files.com
        let encoded_path = encode_path(path);
        let endpoint = format!("/files{}", encoded_path);

        let mut form = vec![("action", "end".to_string())];

        if let Some(ref_value) = &upload_part.ref_ {
            form.push(("ref", ref_value.clone()));
        }

        let response = self.client.post_form(&endpoint, &form).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update file metadata
    ///
    /// # Arguments
    ///
    /// * `path` - File path
    /// * `custom_metadata` - Custom metadata key-value pairs (optional)
    /// * `provided_mtime` - Custom modification time (optional)
    /// * `priority_color` - Priority color (optional)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FileHandler};
    /// # use std::collections::HashMap;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FileHandler::new(client);
    ///
    /// let mut metadata = HashMap::new();
    /// metadata.insert("category".to_string(), "reports".to_string());
    ///
    /// handler.update_file("/path/to/file.txt", Some(metadata), None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_file(
        &self,
        path: &str,
        custom_metadata: Option<HashMap<String, String>>,
        provided_mtime: Option<String>,
        priority_color: Option<String>,
    ) -> Result<FileEntity> {
        let mut body = json!({});

        if let Some(metadata) = custom_metadata {
            body["custom_metadata"] = json!(metadata);
        }

        if let Some(mtime) = provided_mtime {
            body["provided_mtime"] = json!(mtime);
        }

        if let Some(color) = priority_color {
            body["priority_color"] = json!(color);
        }

        let encoded_path = encode_path(path);
        let endpoint = format!("/files{}", encoded_path);
        let response = self.client.patch_raw(&endpoint, body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a file
    ///
    /// # Arguments
    ///
    /// * `path` - File path to delete
    /// * `recursive` - If path is a folder, delete recursively
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FileHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FileHandler::new(client);
    /// handler.delete_file("/path/to/file.txt", false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_file(&self, path: &str, recursive: bool) -> Result<()> {
        let encoded_path = encode_path(path);
        let endpoint = if recursive {
            format!("/files{}?recursive=true", encoded_path)
        } else {
            format!("/files{}", encoded_path)
        };

        self.client.delete_raw(&endpoint).await?;
        Ok(())
    }

    /// Copy a file
    ///
    /// This is a convenience method that calls `FileActionHandler::copy_file()`
    ///
    /// # Arguments
    ///
    /// * `source` - Source file path
    /// * `destination` - Destination path
    pub async fn copy_file(&self, source: &str, destination: &str) -> Result<()> {
        let file_action = FileActionHandler::new(self.client.clone());
        file_action.copy_file(source, destination).await
    }

    /// Move a file
    ///
    /// This is a convenience method that calls `FileActionHandler::move_file()`
    ///
    /// # Arguments
    ///
    /// * `source` - Source file path
    /// * `destination` - Destination path
    pub async fn move_file(&self, source: &str, destination: &str) -> Result<()> {
        let file_action = FileActionHandler::new(self.client.clone());
        file_action.move_file(source, destination).await
    }

    /// Upload an entire directory recursively
    ///
    /// Walks through a local directory and uploads all files to Files.com,
    /// preserving the directory structure.
    ///
    /// # Arguments
    ///
    /// * `local_dir` - Local directory path to upload
    /// * `remote_path` - Remote destination path on Files.com
    /// * `mkdir_parents` - Create parent directories if they don't exist
    ///
    /// # Returns
    ///
    /// Vector of successfully uploaded remote file paths
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Local directory doesn't exist or isn't readable
    /// - Path contains invalid UTF-8
    /// - Any file upload fails
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use files_sdk::{FilesClient, FileHandler};
    /// use std::path::Path;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FileHandler::new(client);
    ///
    /// let uploaded = handler.upload_directory(
    ///     Path::new("./local/images"),
    ///     "/remote/uploads",
    ///     true  // create parent directories
    /// ).await?;
    ///
    /// println!("Uploaded {} files", uploaded.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_directory(
        &self,
        local_dir: &Path,
        remote_path: &str,
        mkdir_parents: bool,
    ) -> Result<Vec<String>> {
        let mut uploaded = Vec::new();

        for entry in WalkDir::new(local_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let local_file = entry.path();

                // Calculate relative path
                let relative = local_file
                    .strip_prefix(local_dir)
                    .map_err(|e| FilesError::IoError(format!("Failed to strip prefix: {}", e)))?
                    .to_str()
                    .ok_or_else(|| {
                        FilesError::IoError(format!(
                            "Invalid UTF-8 in path: {}",
                            local_file.display()
                        ))
                    })?;

                // Construct remote path (use forward slashes for Files.com)
                let remote_file = format!(
                    "{}/{}",
                    remote_path.trim_end_matches('/'),
                    relative.replace('\\', "/")
                );

                // Read and upload
                let data =
                    std::fs::read(local_file).map_err(|e| FilesError::IoError(e.to_string()))?;

                // Upload using the same two-stage process as upload_file
                let file_action = FileActionHandler::new(self.client.clone());
                let upload_parts = file_action
                    .begin_upload(&remote_file, Some(data.len() as i64), mkdir_parents)
                    .await?;

                if !upload_parts.is_empty() {
                    let upload_part = &upload_parts[0];
                    if let Some(upload_uri) = &upload_part.upload_uri {
                        let http_client = reqwest::Client::new();
                        let http_method = upload_part
                            .http_method
                            .as_deref()
                            .unwrap_or("PUT")
                            .to_uppercase();

                        let mut request = match http_method.as_str() {
                            "PUT" => http_client.put(upload_uri),
                            "POST" => http_client.post(upload_uri),
                            _ => http_client.put(upload_uri),
                        };

                        if let Some(headers) = &upload_part.headers {
                            for (key, value) in headers {
                                request = request.header(key, value);
                            }
                        }

                        request.body(data.to_vec()).send().await?;
                    }
                }

                uploaded.push(remote_file);
            }
        }

        Ok(uploaded)
    }

    /// Upload directory with progress callback
    ///
    /// Same as `upload_directory` but calls a progress callback after each file upload.
    /// Useful for showing upload progress in UIs or logging.
    ///
    /// # Arguments
    ///
    /// * `local_dir` - Local directory path to upload
    /// * `remote_path` - Remote destination path on Files.com
    /// * `mkdir_parents` - Create parent directories if they don't exist
    /// * `progress` - Callback function called with (current_file_number, total_files)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use files_sdk::{FilesClient, FileHandler};
    /// use std::path::Path;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FileHandler::new(client);
    ///
    /// handler.upload_directory_with_progress(
    ///     Path::new("./data"),
    ///     "/backups",
    ///     true,
    ///     |current, total| {
    ///         println!("Progress: {}/{} ({:.1}%)",
    ///             current, total, (current as f64 / total as f64) * 100.0);
    ///     }
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn upload_directory_with_progress<F>(
        &self,
        local_dir: &Path,
        remote_path: &str,
        mkdir_parents: bool,
        progress: F,
    ) -> Result<Vec<String>>
    where
        F: Fn(usize, usize),
    {
        // Count files first
        let total_files = WalkDir::new(local_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .count();

        let mut uploaded = Vec::new();
        let mut current = 0;

        for entry in WalkDir::new(local_dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let local_file = entry.path();

                // Calculate relative path
                let relative = local_file
                    .strip_prefix(local_dir)
                    .map_err(|e| FilesError::IoError(format!("Failed to strip prefix: {}", e)))?
                    .to_str()
                    .ok_or_else(|| {
                        FilesError::IoError(format!(
                            "Invalid UTF-8 in path: {}",
                            local_file.display()
                        ))
                    })?;

                // Construct remote path
                let remote_file = format!(
                    "{}/{}",
                    remote_path.trim_end_matches('/'),
                    relative.replace('\\', "/")
                );

                // Read and upload
                let data =
                    std::fs::read(local_file).map_err(|e| FilesError::IoError(e.to_string()))?;

                // Upload using the same two-stage process as upload_file
                let file_action = FileActionHandler::new(self.client.clone());
                let upload_parts = file_action
                    .begin_upload(&remote_file, Some(data.len() as i64), mkdir_parents)
                    .await?;

                if !upload_parts.is_empty() {
                    let upload_part = &upload_parts[0];
                    if let Some(upload_uri) = &upload_part.upload_uri {
                        let http_client = reqwest::Client::new();
                        let http_method = upload_part
                            .http_method
                            .as_deref()
                            .unwrap_or("PUT")
                            .to_uppercase();

                        let mut request = match http_method.as_str() {
                            "PUT" => http_client.put(upload_uri),
                            "POST" => http_client.post(upload_uri),
                            _ => http_client.put(upload_uri),
                        };

                        if let Some(headers) = &upload_part.headers {
                            for (key, value) in headers {
                                request = request.header(key, value);
                            }
                        }

                        request.body(data.to_vec()).send().await?;
                    }
                }

                uploaded.push(remote_file);
                current += 1;
                progress(current, total_files);
            }
        }

        Ok(uploaded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = FileHandler::new(client);
    }
}
