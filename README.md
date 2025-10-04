# files-sdk

Rust SDK for the [Files.com](https://files.com) REST API.

> **Status**: 🚧 Alpha - Full API coverage (288 endpoints), core functionality tested

## Overview

Comprehensive, idiomatic Rust SDK for Files.com cloud storage platform. Provides type-safe, async operations across the entire Files.com API including file operations, user management, sharing, automation, and administration.

## Installation

Not yet published to crates.io. Add via git:

```toml
[dependencies]
files-sdk = { git = "https://github.com/joshrotenberg/files-sdk" }

# Optional: Enable tracing for HTTP-level debugging
files-sdk = { git = "https://github.com/joshrotenberg/files-sdk", features = ["tracing"] }
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
    let data = b"Hello, Files.com!";
    
    let result = file_handler
        .upload_file("/path/to/file.txt", data)
        .await?;
    
    println!("Uploaded: {:?}", result.path);
    Ok(())
}
```

## Features

### Core Capabilities
- ✅ **Complete API Coverage** - All 288 endpoints across 90+ handlers
- ✅ **Type Safety** - Full Rust type system with Result-based error handling
- ✅ **Async/Await** - Built on tokio for efficient async operations
- ✅ **Builder Pattern** - Ergonomic client configuration
- ✅ **Comprehensive Errors** - 14 error types matching HTTP status codes
- ✅ **Pagination** - Cursor-based pagination support
- ✅ **Tracing** - Optional HTTP-level debugging (feature: `tracing`)

### API Modules

**Files** (`files::`)
- File upload/download, copy, move, delete
- Folder operations with recursive support
- File comments and reactions
- File migrations

**Users** (`users::`)
- User management, groups, permissions
- API keys, sessions, public keys
- Group memberships

**Sharing** (`sharing::`)
- Bundles (share links)
- File requests, inbox uploads
- Bundle notifications and recipients

**Automation** (`automation::`)
- Automations and automation runs
- Behaviors (webhooks, auto-encrypt)
- Remote servers and syncs

**Admin** (`admin::`)
- Site settings and configuration
- History, invoices, payments
- Usage statistics

**Logs** (`logs::`)
- API request logs, SFTP/FTP action logs
- Automation logs, sync logs
- Settings change tracking

**Messages** (`messages::`)
- Messages and notifications
- Notification exports

**Storage** (`storage::`)
- Projects, snapshots, locks
- File priorities

**Security** (`security::`)
- GPG keys, SFTP host keys
- Clickwraps

**AS2** (`as2::`)
- AS2 stations, partners, keys
- Incoming/outgoing messages

**Advanced** (`advanced::`)
- Form field sets, share groups
- SIEM HTTP destinations

### Optional Tracing

Enable detailed HTTP debugging:

```rust
use files_sdk::{FilesClient, files::FileHandler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("files_sdk=debug")
        .init();

    let client = FilesClient::builder()
        .api_key("your-api-key")
        .build()?;

    // All HTTP requests logged
    let handler = FileHandler::new(client);
    handler.upload_file("/test.txt", b"data").await?;
    
    Ok(())
}
```

Control verbosity:
```bash
RUST_LOG=files_sdk=debug cargo run
RUST_LOG=files_sdk=trace cargo run
```

## Architecture

**Domain-Driven Organization**
```
files_sdk::
├── files::       File and folder operations
├── users::       User and access management
├── sharing::     Bundles and file requests
├── automation::  Automations and behaviors
├── admin::       Site administration
├── logs::        Activity logging
├── messages::    Notifications
├── storage::     Projects and snapshots
├── security::    Keys and authentication
├── as2::         AS2 protocol support
└── advanced::    Advanced features
```

**Low-Level API** - Direct handler access for full control
```rust
use files_sdk::{FilesClient, files::FileHandler};

let client = FilesClient::builder().api_key("key").build()?;
let handler = FileHandler::new(client);
handler.upload_file("/path", data).await?;
```

## Testing

**56 Integration Tests** across core modules:
- Files: 38 tests (upload, download, folders, comments)
- Users: 12 tests (users, groups, API keys, sessions)
- Sharing: 3 tests (bundles)
- Admin: 2 tests (site settings)
- Automation: 4 tests (automations, behaviors)

**177 Mock Tests** providing comprehensive unit coverage

Run tests:
```bash
# Unit tests
cargo test --lib

# Mock tests
cargo test --test mock

# Integration tests (requires FILES_API_KEY)
FILES_API_KEY=your_key cargo test --test real --features integration-tests
```

## Error Handling

Comprehensive error types:
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

## Development Status

### ✅ Complete
- Full API coverage (288 endpoints, 90 handlers)
- Type-safe client with builder pattern
- Comprehensive error handling
- Pagination support
- Integration test framework
- Optional tracing
- Mock test suite

### 🚧 In Progress
- Files.com account for real API testing
- Performance optimization
- Additional examples

### 📋 Planned
- High-level convenience APIs
- Retry logic with exponential backoff
- crates.io publication
- Complete documentation

## Contributing

This SDK is in active development. Contributions welcome!

## License

MIT OR Apache-2.0
