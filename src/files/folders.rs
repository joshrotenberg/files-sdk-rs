//! Folder operations
//!
//! Provides directory management and navigation operations for Files.com. Folders are
//! represented as `FileEntity` objects with `type="directory"`.
//!
//! # Features
//!
//! - List folder contents with pagination
//! - Create folders (with parent directory creation)
//! - Delete folders (recursive or non-recursive)
//! - Search files within folders
//! - Automatic pagination for large directories
//!
//! # Example
//!
//! ```no_run
//! use files_sdk::{FilesClient, FolderHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = FilesClient::builder()
//!     .api_key("your-api-key")
//!     .build()?;
//!
//! let handler = FolderHandler::new(client);
//!
//! // List root directory
//! let (files, pagination) = handler.list_folder("/", None, None).await?;
//! for file in files {
//!     println!("{}: {}",
//!         file.file_type.unwrap_or_default(),
//!         file.path.unwrap_or_default());
//! }
//!
//! // Create a new folder with parent directories
//! handler.create_folder("/projects/2024/q4", true).await?;
//!
//! // Search for files
//! let (results, _) = handler.search_folder("/", "report", None).await?;
//! println!("Found {} matching files", results.len());
//! # Ok(())
//! # }
//! ```

use crate::utils::encode_path;
use crate::{FileEntity, FilesClient, PaginationInfo, Result};
use futures::stream::Stream;
use serde_json::json;

/// Handler for folder operations
///
/// Provides methods for listing, creating, searching, and managing folders
/// (directories) in Files.com.
#[derive(Debug, Clone)]
pub struct FolderHandler {
    client: FilesClient,
}

impl FolderHandler {
    /// Creates a new FolderHandler
    ///
    /// # Arguments
    ///
    /// * `client` - FilesClient instance
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List folder contents
    ///
    /// Returns files and subdirectories within the specified folder.
    ///
    /// # Arguments
    ///
    /// * `path` - Folder path to list (empty string for root)
    /// * `per_page` - Number of items per page (optional, max 10,000)
    /// * `cursor` - Pagination cursor (optional)
    ///
    /// # Returns
    ///
    /// Returns a tuple of (files, pagination_info)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use files_sdk::{FilesClient, FolderHandler};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder()
    ///     .api_key("your-api-key")
    ///     .build()?;
    ///
    /// let handler = FolderHandler::new(client);
    /// let (files, pagination) = handler.list_folder("/", None, None).await?;
    ///
    /// for file in files {
    ///     println!("{}: {}", file.file_type.unwrap_or_default(), file.path.unwrap_or_default());
    /// }
    ///
    /// if pagination.has_next() {
    ///     println!("More results available");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_folder(
        &self,
        path: &str,
        per_page: Option<i32>,
        cursor: Option<String>,
    ) -> Result<(Vec<FileEntity>, PaginationInfo)> {
        let encoded_path = encode_path(path);
        let mut endpoint = format!("/folders{}", encoded_path);
        let mut query_params = Vec::new();

        if let Some(per_page) = per_page {
            query_params.push(format!("per_page={}", per_page));
        }

        if let Some(cursor) = cursor {
            query_params.push(format!("cursor={}", cursor));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        // Need to get the raw response to access headers
        let url = format!("{}{}", self.client.inner.base_url, endpoint);
        let response = reqwest::Client::new()
            .get(&url)
            .header("X-FilesAPI-Key", &self.client.inner.api_key)
            .send()
            .await?;

        let headers = response.headers().clone();
        let pagination = PaginationInfo::from_headers(&headers);

        let status = response.status();
        if !status.is_success() {
            return Err(crate::FilesError::ApiError {
                endpoint: None,
                code: status.as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let files: Vec<FileEntity> = response.json().await?;

        Ok((files, pagination))
    }

    /// List all folder contents (auto-pagination)
    ///
    /// Automatically handles pagination to retrieve all items in a folder.
    ///
    /// # Arguments
    ///
    /// * `path` - Folder path to list
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FolderHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FolderHandler::new(client);
    /// let all_files = handler.list_folder_all("/uploads").await?;
    /// println!("Total files: {}", all_files.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_folder_all(&self, path: &str) -> Result<Vec<FileEntity>> {
        let mut all_files = Vec::new();
        let mut cursor = None;

        loop {
            let (mut files, pagination) = self.list_folder(path, Some(1000), cursor).await?;
            all_files.append(&mut files);

            if pagination.has_next() {
                cursor = pagination.cursor_next;
            } else {
                break;
            }
        }

        Ok(all_files)
    }

    /// Stream folder contents with automatic pagination
    ///
    /// Returns a stream that automatically handles pagination, yielding
    /// individual files as they are fetched. This is more memory-efficient
    /// than `list_folder_all()` for large directories.
    ///
    /// # Arguments
    ///
    /// * `path` - Folder path to list
    /// * `per_page` - Number of items per page (optional, default 1000)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FolderHandler};
    /// # use futures::stream::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FolderHandler::new(client);
    /// let stream = handler.list_stream("/uploads", Some(100));
    ///
    /// tokio::pin!(stream);
    ///
    /// while let Some(file) = stream.next().await {
    ///     let file = file?;
    ///     println!("{}", file.path.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_stream(
        &self,
        path: &str,
        per_page: Option<i32>,
    ) -> impl Stream<Item = Result<FileEntity>> + '_ {
        let path = path.to_string();
        let per_page = per_page.unwrap_or(1000);

        async_stream::try_stream! {
            let mut cursor: Option<String> = None;

            loop {
                let (files, pagination) = self
                    .list_folder(&path, Some(per_page), cursor.clone())
                    .await?;

                for file in files {
                    yield file;
                }

                match pagination.cursor_next {
                    Some(next) => cursor = Some(next),
                    None => break,
                }
            }
        }
    }

    /// Create a new folder
    ///
    /// Note: In Files.com, folders are created implicitly when uploading files
    /// with `mkdir_parents=true`. This method creates an empty folder.
    ///
    /// # Arguments
    ///
    /// * `path` - Folder path to create
    /// * `mkdir_parents` - Create parent directories if they don't exist
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FolderHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FolderHandler::new(client);
    /// handler.create_folder("/new/folder", true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_folder(&self, path: &str, mkdir_parents: bool) -> Result<FileEntity> {
        let body = json!({
            "path": path,
            "mkdir_parents": mkdir_parents,
        });

        let encoded_path = encode_path(path);
        let endpoint = format!("/folders{}", encoded_path);
        let response = self.client.post_raw(&endpoint, body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a folder
    ///
    /// # Arguments
    ///
    /// * `path` - Folder path to delete
    /// * `recursive` - Delete folder and all contents recursively
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FolderHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FolderHandler::new(client);
    /// handler.delete_folder("/old/folder", true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_folder(&self, path: &str, recursive: bool) -> Result<()> {
        let encoded_path = encode_path(path);
        let endpoint = if recursive {
            format!("/folders{}?recursive=true", encoded_path)
        } else {
            format!("/folders{}", encoded_path)
        };

        self.client.delete_raw(&endpoint).await?;
        Ok(())
    }

    /// Search for files within a folder
    ///
    /// # Arguments
    ///
    /// * `path` - Folder path to search in
    /// * `search` - Search query string
    /// * `per_page` - Number of results per page (optional)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FolderHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FolderHandler::new(client);
    /// let (results, _) = handler.search_folder("/", "report", None).await?;
    /// println!("Found {} files", results.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_folder(
        &self,
        path: &str,
        search: &str,
        per_page: Option<i32>,
    ) -> Result<(Vec<FileEntity>, PaginationInfo)> {
        let encoded_path = encode_path(path);
        let mut endpoint = format!("/folders{}?search={}", encoded_path, search);

        if let Some(per_page) = per_page {
            endpoint.push_str(&format!("&per_page={}", per_page));
        }

        let url = format!("{}{}", self.client.inner.base_url, endpoint);
        let response = reqwest::Client::new()
            .get(&url)
            .header("X-FilesAPI-Key", &self.client.inner.api_key)
            .send()
            .await?;

        let headers = response.headers().clone();
        let pagination = PaginationInfo::from_headers(&headers);

        let status = response.status();
        if !status.is_success() {
            return Err(crate::FilesError::ApiError {
                endpoint: None,
                code: status.as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let files: Vec<FileEntity> = response.json().await?;

        Ok((files, pagination))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = FolderHandler::new(client);
    }
}
