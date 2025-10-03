# files-sdk

Rust SDK for [Files.com](https://files.com) REST API.

> **Status**: 🚧 Early Development - Skeleton only

## Purpose

Enable Rust applications to upload/download files to Files.com cloud storage. Primary use case is uploading Redis Enterprise support packages from `redisctl`.

## Installation

Not yet published to crates.io.

## Quick Start (Planned)

```rust
use files_sdk::FilesClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FilesClient::new("your-api-key")?;
    let data = b"Hello, Files.com!";
    let response = client.upload_bytes("/path/to/file.txt", data).await?;
    println!("Uploaded: {}", response.path);
    Ok(())
}
```

## Development Status

Currently just a skeleton. Next steps:
1. Research Files.com REST API
2. Implement upload functionality
3. Add tests
4. Integrate with redisctl

See `CLAUDE.md` for detailed project context.
