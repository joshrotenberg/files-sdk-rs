//! Common types for Files.com API
//!
//! This module contains shared types used across multiple endpoints,
//! including file entities, folder entities, pagination information,
//! and upload-related types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a file or directory in Files.com
///
/// This is the primary entity returned by most file operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntity {
    /// File/folder path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Display name of file/folder
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Type: "file" or "directory"
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,

    /// Size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,

    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Modification time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<String>,

    /// Provided modification time (custom)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_mtime: Option<String>,

    /// CRC32 checksum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crc32: Option<String>,

    /// MD5 hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,

    /// SHA1 hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,

    /// SHA256 hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,

    /// MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// Storage region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Permissions string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<String>,

    /// Whether subfolders are locked
    #[serde(rename = "subfolders_locked?", skip_serializing_if = "Option::is_none")]
    pub subfolders_locked: Option<bool>,

    /// Whether file is locked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,

    /// Download URI (temporary URL for downloading)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,

    /// Priority color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_color: Option<String>,

    /// Preview ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_id: Option<i64>,

    /// Preview information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,

    /// Custom metadata (max 32 keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata: Option<HashMap<String, String>>,

    /// ID of user who created this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_id: Option<i64>,

    /// ID of API key that created this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_api_key_id: Option<i64>,

    /// ID of automation that created this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_automation_id: Option<i64>,

    /// ID of bundle registration that created this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_bundle_registration_id: Option<i64>,

    /// ID of inbox that created this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_inbox_id: Option<i64>,

    /// ID of remote server that created this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_remote_server_id: Option<i64>,

    /// ID of remote server sync that created this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_remote_server_sync_id: Option<i64>,

    /// ID of AS2 incoming message that created this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_as2_incoming_message_id: Option<i64>,

    /// ID of user who last modified this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by_id: Option<i64>,

    /// ID of API key that last modified this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by_api_key_id: Option<i64>,

    /// ID of automation that last modified this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by_automation_id: Option<i64>,

    /// ID of bundle registration that last modified this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by_bundle_registration_id: Option<i64>,

    /// ID of remote server that last modified this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by_remote_server_id: Option<i64>,

    /// ID of remote server sync that last modified this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by_remote_server_sync_id: Option<i64>,
}

/// Represents upload information for a file part
///
/// Returned by the begin_upload operation to provide URLs and parameters
/// for uploading file data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadPartEntity {
    /// URI to upload this part to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_uri: Option<String>,

    /// HTTP method to use (usually "PUT")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,

    /// Additional headers to include in upload request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// Additional HTTP parameters to send
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, String>>,

    /// Part number for multi-part uploads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<i32>,

    /// Size in bytes for this part
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partsize: Option<i64>,

    /// Size in bytes for the next part
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_partsize: Option<i64>,

    /// Reference identifier for this upload
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,

    /// Type of upload action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    /// Whether multiple parts can be uploaded in parallel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_parts: Option<bool>,

    /// Whether parts can be retried
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_parts: Option<bool>,

    /// Number of parts in the upload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_parts: Option<i32>,

    /// When this upload URL expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,

    /// Content-Type and file to send
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send: Option<HashMap<String, String>>,

    /// File path being uploaded to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Whether to ask about overwrites
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_about_overwrites: Option<bool>,
}

/// Represents a folder (directory) in Files.com
///
/// Alias for FileEntity since folders are represented as files with type="directory"
pub type FolderEntity = FileEntity;

/// Pagination information from response headers
///
/// Files.com uses cursor-based pagination with cursors provided in response headers.
#[derive(Debug, Clone, Default)]
pub struct PaginationInfo {
    /// Cursor for the next page of results
    pub cursor_next: Option<String>,

    /// Cursor for the previous page of results
    pub cursor_prev: Option<String>,
}

impl PaginationInfo {
    /// Creates pagination info from response headers
    pub fn from_headers(headers: &reqwest::header::HeaderMap) -> Self {
        let cursor_next = headers
            .get("X-Files-Cursor-Next")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let cursor_prev = headers
            .get("X-Files-Cursor-Prev")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        Self {
            cursor_next,
            cursor_prev,
        }
    }

    /// Whether there is a next page
    pub fn has_next(&self) -> bool {
        self.cursor_next.is_some()
    }

    /// Whether there is a previous page
    pub fn has_prev(&self) -> bool {
        self.cursor_prev.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_entity_deserialize() {
        let json = r#"{
            "path": "/test/file.txt",
            "display_name": "file.txt",
            "type": "file",
            "size": 1024
        }"#;

        let entity: FileEntity = serde_json::from_str(json).unwrap();
        assert_eq!(entity.path, Some("/test/file.txt".to_string()));
        assert_eq!(entity.display_name, Some("file.txt".to_string()));
        assert_eq!(entity.file_type, Some("file".to_string()));
        assert_eq!(entity.size, Some(1024));
    }

    #[test]
    fn test_pagination_info_empty() {
        let headers = reqwest::header::HeaderMap::new();
        let info = PaginationInfo::from_headers(&headers);
        assert!(!info.has_next());
        assert!(!info.has_prev());
    }

    #[test]
    fn test_pagination_info_with_next() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("X-Files-Cursor-Next", "next-cursor".parse().unwrap());

        let info = PaginationInfo::from_headers(&headers);
        assert!(info.has_next());
        assert!(!info.has_prev());
        assert_eq!(info.cursor_next, Some("next-cursor".to_string()));
    }
}
