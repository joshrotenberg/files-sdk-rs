# files-sdk

[![Crates.io](https://img.shields.io/crates/v/files-sdk.svg)](https://crates.io/crates/files-sdk)
[![Documentation](https://docs.rs/files-sdk/badge.svg)](https://docs.rs/files-sdk)
[![License](https://img.shields.io/crates/l/files-sdk.svg)](https://github.com/joshrotenberg/files-sdk-rs#license)
[![CI](https://github.com/joshrotenberg/files-sdk-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/joshrotenberg/files-sdk-rs/actions/workflows/ci.yml)

A Rust SDK for the [Files.com](https://files.com) API.

> **Note**: This is an unofficial, community-maintained library and is not supported by Files.com. For official SDKs, see the [Files.com Developer Documentation](https://developers.files.com/).

## About Files.com

[Files.com](https://files.com) is a cloud storage and file transfer platform. This SDK provides access to the [Files.com REST API](https://developers.files.com/api/)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
files-sdk = "0.3"
```

## Quick Start

```rust
use files_sdk::{FilesClient, files::FileHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FilesClient::builder()
        .api_key("your-api-key")
        .build()?;

    let file_handler = FileHandler::new(client);
    
    // Upload a file
    let data = b"Hello, Files.com!";
    file_handler.upload_file("/path/to/file.txt", data).await?;
    
    // Download a file
    let content = file_handler.download("/path/to/file.txt").await?;
    
    // List folder contents
    let (files, _pagination) = file_handler.list_folder("/", None, Some(100)).await?;
    for file in files {
        println!("{}: {} bytes", file.path, file.size);
    }
    
    Ok(())
}
```

## Examples

### File Operations

```rust
use files_sdk::{FilesClient, files::FileHandler};
use std::path::Path;

let client = FilesClient::builder().api_key("key").build()?;
let handler = FileHandler::new(client);

// Upload with automatic parent directory creation
handler.upload_file_with_options(
    "/reports/2024/summary.pdf",
    data,
    true  // mkdir_parents
).await?;

// Download file metadata (returns FileEntity with download_uri)
let file = handler.download_file("/reports/2024/summary.pdf").await?;
println!("Download URL: {:?}", file.download_uri);

// Download actual file content as bytes
let content = handler.download_content("/reports/2024/summary.pdf").await?;
println!("Downloaded {} bytes", content.len());

// Download directly to local file
handler.download_to_file(
    "/reports/2024/summary.pdf",
    Path::new("./local/summary.pdf")
).await?;

// Copy file
handler.copy_file("/original.txt", "/backup.txt").await?;

// Move file
handler.move_file("/old/path.txt", "/new/path.txt").await?;

// Delete file
handler.delete_file("/unwanted.txt", false).await?;

// Upload entire directory recursively
let uploaded = handler.upload_directory(
    Path::new("./local/images"),
    "/remote/uploads",
    true  // create parent directories
).await?;
println!("Uploaded {} files: {:?}", uploaded.len(), uploaded);

// Upload directory with progress callback
handler.upload_directory_with_progress(
    Path::new("./data"),
    "/backups",
    true,
    |current, total| {
        println!("Progress: {}/{} ({:.1}%)",
            current, total, (current as f64 / total as f64) * 100.0);
    }
).await?;
```

### User Management

```rust
use files_sdk::{FilesClient, users::UserHandler};

let client = FilesClient::builder().api_key("key").build()?;
let handler = UserHandler::new(client);

// List users with pagination
let (users, pagination) = handler.list(None, Some(50)).await?;

// Get specific user
let user = handler.get(123).await?;

// Create user
let new_user = handler.create("user@example.com", "username", None).await?;

// Update user
handler.update(123, Some("new@example.com"), None).await?;
```

### File Sharing

```rust
use files_sdk::{FilesClient, sharing::BundleHandler};

let client = FilesClient::builder().api_key("key").build()?;
let handler = BundleHandler::new(client);

// Create share link
let bundle = handler.create(
    vec!["/reports/Q4.pdf".to_string()],
    None,  // password
    Some(7)  // expires in 7 days
).await?;

println!("Share URL: {}", bundle.url);

// List all bundles
let (bundles, _) = handler.list(None, Some(100)).await?;
```

### Automation

```rust
use files_sdk::{FilesClient, automation::AutomationHandler};

let client = FilesClient::builder().api_key("key").build()?;
let handler = AutomationHandler::new(client);

// Create automation
let automation = handler.create(
    "folder_sync",  // automation type
    Some("/sync/*"),  // path
    None  // additional options
).await?;

// List automations
let (automations, _) = handler.list(None, Some(50), None).await?;
```

### Pagination

The SDK provides three approaches to handle paginated results:

#### Manual Pagination

```rust
use files_sdk::{FilesClient, FolderHandler};

let client = FilesClient::builder().api_key("key").build()?;
let handler = FolderHandler::new(client);

// Get first page
let (files, pagination) = handler.list_folder("/uploads", Some(100), None).await?;

// Get next page if available
if let Some(cursor) = pagination.cursor_next {
    let (more_files, _) = handler.list_folder("/uploads", Some(100), Some(cursor)).await?;
}
```

#### Auto-Pagination (Collect All)

```rust
use files_sdk::{FilesClient, FolderHandler};

let client = FilesClient::builder().api_key("key").build()?;
let handler = FolderHandler::new(client);

// Automatically fetch all pages
let all_files = handler.list_folder_all("/uploads").await?;
println!("Total files: {}", all_files.len());
```

#### Streaming Pagination (Memory-Efficient)

```rust
use files_sdk::{FilesClient, FolderHandler, UserHandler};
use futures::stream::StreamExt;

let client = FilesClient::builder().api_key("key").build()?;

// Stream folder contents
let folder_handler = FolderHandler::new(client.clone());
let mut stream = folder_handler.list_stream("/uploads", Some(100));

while let Some(file) = stream.next().await {
    let file = file?;
    println!("Processing: {}", file.path.unwrap_or_default());
}

// Or collect all at once
let stream = folder_handler.list_stream("/uploads", Some(100));
let all_files: Vec<_> = stream.try_collect().await?;

// Stream users
let user_handler = UserHandler::new(client);
let mut user_stream = user_handler.list_stream(Some(50));

while let Some(user) = user_stream.next().await {
    let user = user?;
    println!("User: {}", user.username.unwrap_or_default());
}
```

**When to use each approach:**
- **Manual**: Fine-grained control, show "Load More" UI
- **Auto-pagination**: Simple cases, small-to-medium result sets
- **Streaming**: Large result sets, memory-constrained environments, real-time processing

### Error Handling

All errors include contextual information to help with debugging and recovery:

```rust
use files_sdk::{FilesClient, FilesError, files::FileHandler};

let client = FilesClient::builder().api_key("key").build()?;
let handler = FileHandler::new(client);

match handler.download_content("/reports/missing.pdf").await {
    Ok(content) => println!("Downloaded {} bytes", content.len()),
    
    Err(FilesError::NotFound { message, resource_type, path, .. }) => {
        eprintln!("Not found: {}", message);
        if let Some(rt) = resource_type {
            eprintln!("  Resource type: {}", rt);
        }
        if let Some(p) = path {
            eprintln!("  Path: {}", p);
        }
    }
    
    Err(FilesError::RateLimited { message, retry_after, .. }) => {
        eprintln!("Rate limited: {}", message);
        if let Some(seconds) = retry_after {
            eprintln!("  Retry after {} seconds", seconds);
        }
    }
    
    Err(FilesError::UnprocessableEntity { message, field, value, .. }) => {
        eprintln!("Validation failed: {}", message);
        if let Some(f) = field {
            eprintln!("  Invalid field: {}", f);
        }
        if let Some(v) = value {
            eprintln!("  Invalid value: {}", v);
        }
    }
    
    Err(e) => {
        eprintln!("Error: {}", e);
        
        // Check if error is retryable
        if e.is_retryable() {
            eprintln!("  This error may be temporary - consider retrying");
        }
        
        // Get HTTP status code if available
        if let Some(code) = e.status_code() {
            eprintln!("  HTTP status: {}", code);
        }
    }
}
```

Helper methods for error construction:

```rust
use files_sdk::FilesError;

// Create contextual errors
let err = FilesError::not_found_resource(
    "File does not exist",
    "file",
    "/reports/Q4.pdf"
);

let err = FilesError::validation_failed(
    "Invalid email format",
    "email",
    "not-an-email"
);

let err = FilesError::rate_limited("Too many requests", Some(60));
```

### Tracing (Optional)

Enable HTTP-level debugging:

```toml
[dependencies]
files-sdk = { version = "0.1", features = ["tracing"] }
tracing-subscriber = "0.3"
```

```rust
use files_sdk::{FilesClient, files::FileHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("files_sdk=debug")
        .init();

    let client = FilesClient::builder().api_key("key").build()?;
    let handler = FileHandler::new(client);
    
    // All HTTP requests/responses logged
    handler.upload_file("/test.txt", b"data").await?;
    
    Ok(())
}
```

```bash
RUST_LOG=files_sdk=debug cargo run
```

## API Coverage

| Module | Description |
|--------|-------------|
| `files::` | File upload/download, folders, comments |
| `users::` | Users, groups, permissions, API keys |
| `sharing::` | Bundles, file requests, share groups, forms |
| `automation::` | Automations, behaviors, webhooks |
| `admin::` | Site settings, history, invoices, DNS, styles |
| `logs::` | API logs, SFTP logs, audit trails, external events |
| `messages::` | Notifications, message exports |
| `storage::` | Projects, snapshots, locks |
| `security::` | GPG keys, SFTP host keys |
| `as2::` | AS2 stations, partners, messages |
| `integrations::` | SIEM destinations |
| `developers::` | Apps and API integrations |

## Path Encoding & Special Characters

The SDK automatically handles special characters in file paths:

```rust
// These all work correctly - paths are automatically URL-encoded
handler.upload_file("/my folder/file.txt", data).await?;           // spaces
handler.upload_file("/data/file[2024].txt", data).await?;          // brackets
handler.upload_file("/文档/测试.txt", data).await?;                 // unicode
handler.upload_file("/files/report@#1.txt", data).await?;          // special chars
```

Paths are encoded using percent-encoding (RFC 3986), ensuring compatibility with Files.com's API regardless of the characters used in file or folder names.

## Error Types

All errors include optional contextual fields for better debugging:

```rust
pub enum FilesError {
    BadRequest { 
        message: String,
        field: Option<String>,  // Which field caused the error
    },
    
    AuthenticationFailed { 
        message: String,
        request_id: Option<String>,  // Request ID for support
    },
    
    Forbidden { 
        message: String,
        resource_type: Option<String>,  // What resource was forbidden
    },
    
    NotFound { 
        message: String,
        resource_type: Option<String>,  // e.g., "file", "user"
        path: Option<String>,           // Path that wasn't found
    },
    
    Conflict { 
        message: String,
        resource_id: Option<String>,    // Conflicting resource ID
    },
    
    PreconditionFailed { 
        message: String,
        condition: Option<String>,      // Which precondition failed
    },
    
    UnprocessableEntity { 
        message: String,
        field: Option<String>,          // Invalid field name
        value: Option<String>,          // Invalid value provided
    },
    
    Locked { 
        message: String,
        path: Option<String>,           // Locked resource path
    },
    
    RateLimited { 
        message: String,
        retry_after: Option<u64>,       // Seconds to wait before retry
    },
    
    InternalError { 
        message: String,
        request_id: Option<String>,     // Request ID for support
    },
    
    ApiError {
        code: u16,
        message: String,
        endpoint: Option<String>,       // Which endpoint failed
    },
    
    // Library errors
    Request(reqwest::Error),
    JsonError(serde_json::Error),
    IoError(std::io::Error),
    BuilderError(String),
    UrlParseError(url::ParseError),
}
```

Utility methods:

```rust
// Get HTTP status code
if let Some(code) = error.status_code() {
    println!("HTTP {}", code);
}

// Check if error is retryable (429, 500, 502, 503, 504)
if error.is_retryable() {
    // Implement retry logic
}

// Get retry delay for rate limits
if let Some(seconds) = error.retry_after() {
    tokio::time::sleep(Duration::from_secs(seconds)).await;
}
```

## Testing

The SDK provides comprehensive testing examples to help you test code that uses Files.com without hitting the real API.

### Running SDK Tests

```bash
# Unit tests
cargo test --lib

# Integration tests (requires FILES_API_KEY)
FILES_API_KEY=your_key cargo test --test real --features integration-tests
```

### Testing Your Code

See `examples/testing/` for complete examples of different testing approaches:

#### 1. Trait-Based Mocking with mockall

Create mockable traits for your file operations:

```rust
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait FileUploader {
    fn upload(&self, path: &str, data: &[u8]) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_upload_called() {
        let mut mock = MockFileUploader::new();
        
        mock.expect_upload()
            .with(eq("/test.txt"), eq(b"data".as_slice()))
            .times(1)
            .returning(|_, _| Ok(()));
        
        // Test your code that uses the mock
        assert!(mock.upload("/test.txt", b"data").is_ok());
    }
}
```

**Run:** `cargo test --example mockall_example`

#### 2. Test Doubles (Hand-Written Fakes)

Build custom test doubles that track state:

```rust
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct FakeFilesClient {
    uploaded_files: Arc<Mutex<Vec<String>>>,
}

impl FakeFilesClient {
    pub fn new() -> Self {
        Self {
            uploaded_files: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn upload(&self, path: &str, data: &[u8]) -> Result<(), String> {
        self.uploaded_files.lock().unwrap().push(path.to_string());
        Ok(())
    }
    
    pub fn get_uploaded_files(&self) -> Vec<String> {
        self.uploaded_files.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracks_uploads() {
        let fake = FakeFilesClient::new();
        fake.upload("/file1.txt", b"data").unwrap();
        fake.upload("/file2.txt", b"data").unwrap();
        
        assert_eq!(fake.get_uploaded_files().len(), 2);
    }
}
```

**Run:** `cargo test --example test_doubles_example`

#### 3. HTTP Mocking with wiremock

Test actual HTTP interactions with a mock server:

```rust
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path, header};
use files_sdk::FilesClient;

#[tokio::test]
async fn test_with_mock_server() {
    // Start mock server
    let mock_server = MockServer::start().await;
    
    // Configure mock response
    Mock::given(method("GET"))
        .and(path("/files/test.txt"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "path": "/test.txt",
                "size": 1024
            })))
        .mount(&mock_server)
        .await;
    
    // Create client pointing to mock server
    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    
    // Test your code
    let result = client.get_raw("/files/test.txt").await;
    assert!(result.is_ok());
}
```

**Run:** `cargo test --example wiremock_example`

### Which Approach to Use?

| Approach | Best For | Pros | Cons |
|----------|----------|------|------|
| **mockall** | Unit tests, verifying method calls | Auto-generated mocks, expectation verification | Requires trait abstraction |
| **Test Doubles** | Integration tests, state verification | No dependencies, full control | More code to maintain |
| **wiremock** | HTTP-level testing, API contract testing | Tests real HTTP flow, verifies headers/body | Slower, more setup |

### Development Dependencies

Add to your `Cargo.toml` for testing:

```toml
[dev-dependencies]
mockall = "0.13"      # For trait-based mocking
wiremock = "0.6"      # For HTTP mocking (already included in SDK)
```

## License

MIT OR Apache-2.0
