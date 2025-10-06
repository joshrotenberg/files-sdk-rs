# files-sdk

[![Crates.io](https://img.shields.io/crates/v/files-sdk.svg)](https://crates.io/crates/files-sdk)
[![Documentation](https://docs.rs/files-sdk/badge.svg)](https://docs.rs/files-sdk)
[![License](https://img.shields.io/crates/l/files-sdk.svg)](https://github.com/joshrotenberg/files-sdk-rs#license)
[![CI](https://github.com/joshrotenberg/files-sdk-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/joshrotenberg/files-sdk-rs/actions/workflows/ci.yml)

Rust SDK for the [Files.com](https://files.com) API - 288 endpoints, fully async, type-safe.

## Installation

```toml
[dependencies]
files-sdk = "0.1"
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

let client = FilesClient::builder().api_key("key").build()?;
let handler = FileHandler::new(client);

// Upload with automatic parent directory creation
handler.upload_file_with_options(
    "/reports/2024/summary.pdf",
    data,
    true  // mkdir_parents
).await?;

// Copy file
handler.copy("/original.txt", "/backup.txt").await?;

// Move file
handler.move_file("/old/path.txt", "/new/path.txt").await?;

// Delete file
handler.delete("/unwanted.txt").await?;
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

### Error Handling

```rust
use files_sdk::{FilesClient, FilesError, files::FileHandler};

let client = FilesClient::builder().api_key("key").build()?;
let handler = FileHandler::new(client);

match handler.download("/missing.txt").await {
    Ok(content) => println!("Downloaded {} bytes", content.len()),
    Err(FilesError::NotFound { message }) => {
        eprintln!("File not found: {}", message);
    }
    Err(FilesError::AuthenticationFailed { message }) => {
        eprintln!("Auth failed: {}", message);
    }
    Err(FilesError::RateLimited { message }) => {
        eprintln!("Rate limited: {}", message);
    }
    Err(e) => eprintln!("Error: {}", e),
}
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

| Module | Endpoints | Description |
|--------|-----------|-------------|
| `files::` | 50+ | File upload/download, folders, comments |
| `users::` | 40+ | Users, groups, permissions, API keys |
| `sharing::` | 35+ | Bundles, file requests, share groups, forms |
| `automation::` | 25+ | Automations, behaviors, webhooks |
| `admin::` | 28+ | Site settings, history, invoices, DNS, styles |
| `logs::` | 35+ | API logs, SFTP logs, audit trails, external events |
| `messages::` | 10+ | Notifications, message exports |
| `storage::` | 15+ | Projects, snapshots, locks |
| `security::` | 10+ | GPG keys, SFTP host keys |
| `as2::` | 40+ | AS2 stations, partners, messages |
| `integrations::` | 5+ | SIEM destinations |
| `developers::` | 5+ | Apps and API integrations |

**Total: 288 endpoints across 90 handlers**

## Error Types

```rust
pub enum FilesError {
    BadRequest { message: String },           // 400
    AuthenticationFailed { message: String }, // 401
    Forbidden { message: String },            // 403
    NotFound { message: String },             // 404
    Conflict { message: String },             // 409
    PreconditionFailed { message: String },   // 412
    UnprocessableEntity { message: String },  // 422
    Locked { message: String },               // 423
    RateLimited { message: String },          // 429
    InternalError { message: String },        // 500+
    Request(reqwest::Error),
    JsonError(serde_json::Error),
    BuilderError(String),
    UrlParseError(url::ParseError),
}
```

## Testing

```bash
# Unit tests
cargo test --lib

# Mock tests
cargo test --test mock

# Integration tests (requires FILES_API_KEY)
FILES_API_KEY=your_key cargo test --test real --features integration-tests
```

## License

MIT OR Apache-2.0
