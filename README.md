# files-sdk

Rust SDK for [Files.com](https://files.com) REST API.

> **Status**: 🚧 Active Development - Core upload functionality working!

## Purpose

Idiomatic Rust SDK for Files.com cloud storage platform. Provides type-safe, async file operations including upload, download, and management.

## Installation

Not yet published to crates.io. Add via git:

```toml
[dependencies]
files-sdk = { git = "https://github.com/joshrotenberg/files-sdk-rs" }
```

## Quick Start

```rust
use files_sdk::{FilesClient, FileHandler};

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

- ✅ **File Upload** - Two-stage upload with automatic S3/cloud storage handling
- ✅ **Type Safety** - Full Rust type system with Result-based error handling
- ✅ **Async/Await** - Built on tokio for efficient async operations
- ✅ **Builder Pattern** - Ergonomic client configuration
- 🚧 **File Download** - Metadata retrieval (content download coming soon)
- 🚧 **Folder Operations** - List, create, delete
- 🚧 **Pagination** - Cursor-based pagination support

## Architecture

Low-level API following Rust idioms:
- Handler structs for resource categories (`FileHandler`, `FolderHandler`)
- Comprehensive error types
- Explicit control over operations

High-level convenience wrappers planned based on common usage patterns.

## Development Status

### Phase 1: Core Infrastructure ✅ COMPLETE
- Client with builder pattern
- File upload (single and binary files, subdirectories)
- Folder operations (basic)
- Error handling

### Next Steps
- Download file content (not just metadata)
- Multi-part uploads for large files
- Additional handlers (users, permissions, etc.)
- Publish to crates.io

## Examples

See `examples/` directory:
- `simple_upload.rs` - Basic file upload
- `test_multiple_uploads.rs` - Various file types and sizes
- `verify_uploads.rs` - Verify uploaded files

Run with: `cargo run --example simple_upload`

## License

MIT OR Apache-2.0
