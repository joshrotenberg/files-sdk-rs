//! Streaming file operations with progress tracking
//!
//! This example demonstrates how to use the streaming API for memory-efficient
//! file uploads and downloads with optional progress tracking.
//!
//! Run with:
//! ```bash
//! FILES_API_KEY=your-key cargo run --example streaming
//! ```

use files_sdk::FilesClient;
use files_sdk::files::FileHandler;
use files_sdk::progress::{Progress, ProgressCallback};
use std::env;
use std::io::Write;
use std::sync::Arc;

use std::time::Instant;

/// Custom progress tracker that displays transfer rate and ETA
struct ProgressTracker {
    start: Instant,
}

impl ProgressTracker {
    fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl ProgressCallback for ProgressTracker {
    fn on_progress(&self, progress: &Progress) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.start).as_secs_f64();

        // Calculate overall transfer rate
        let rate_mbps = if elapsed > 0.0 {
            (progress.bytes_transferred as f64 / elapsed) / (1024.0 * 1024.0)
        } else {
            0.0
        };

        if let Some(total) = progress.total_bytes {
            let pct = progress.percentage().unwrap_or(0.0);
            let remaining_bytes = total.saturating_sub(progress.bytes_transferred);
            let eta_secs = if rate_mbps > 0.0 {
                remaining_bytes as f64 / (rate_mbps * 1024.0 * 1024.0)
            } else {
                0.0
            };

            print!(
                "\r[{:>6.1}%] {:.2} MB / {:.2} MB @ {:.2} MB/s | ETA: {:.0}s    ",
                pct,
                progress.bytes_transferred as f64 / (1024.0 * 1024.0),
                total as f64 / (1024.0 * 1024.0),
                rate_mbps,
                eta_secs
            );
        } else {
            print!(
                "\r{:.2} MB @ {:.2} MB/s    ",
                progress.bytes_transferred as f64 / (1024.0 * 1024.0),
                rate_mbps
            );
        }
        std::io::stdout().flush().unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("FILES_API_KEY").expect("FILES_API_KEY environment variable not set");

    // Create client
    let client = FilesClient::builder().api_key(api_key).build()?;

    let handler = FileHandler::new(client.clone());

    println!("=== Files.com Streaming API Examples ===\n");

    // Example 1: Upload with progress tracking
    println!("1. Uploading file with progress tracking...");
    let upload_path = "/tmp/streaming-example-upload.bin";
    let remote_path = "/streaming-test-upload.bin";

    // Create a 5MB test file
    println!("   Creating 5MB test file...");
    let size = 5 * 1024 * 1024; // 5MB
    let mut file = std::fs::File::create(upload_path)?;
    let chunk = vec![0xAB; 1024]; // 1KB chunk
    for _ in 0..(size / 1024) {
        file.write_all(&chunk)?;
    }
    drop(file);

    // Upload with progress
    println!("   Uploading...");
    let progress_tracker = Arc::new(ProgressTracker::new());
    let file = tokio::fs::File::open(upload_path).await?;

    handler
        .upload_stream(
            remote_path,
            file,
            Some(size as i64),
            Some(progress_tracker.clone()),
        )
        .await?;

    println!("\n   Upload complete!");

    // Example 2: Download with progress tracking
    println!("\n2. Downloading file with progress tracking...");
    let download_path = "/tmp/streaming-example-download.bin";

    println!("   Downloading...");
    let progress_tracker = Arc::new(ProgressTracker::new());
    let mut file = tokio::fs::File::create(download_path).await?;

    handler
        .download_stream(remote_path, &mut file, Some(progress_tracker.clone()))
        .await?;

    println!("\n   Download complete!");

    // Example 3: Upload from memory (Cursor)
    println!("\n3. Uploading from memory buffer...");
    let data = b"Hello from streaming API! This is a test of uploading from memory.";
    let cursor = std::io::Cursor::new(data.to_vec());
    let remote_path_2 = "/streaming-test-memory.txt";

    handler
        .upload_stream(remote_path_2, cursor, Some(data.len() as i64), None)
        .await?;

    println!("   Upload from memory complete!");

    // Example 4: Download to memory
    println!("\n4. Downloading to memory buffer...");
    let mut buffer = Vec::new();

    handler
        .download_stream(remote_path_2, &mut buffer, None)
        .await?;

    println!("   Downloaded {} bytes", buffer.len());
    println!(
        "   Content: {}",
        String::from_utf8_lossy(&buffer[..buffer.len().min(50)])
    );

    // Cleanup
    println!("\n5. Cleaning up test files...");
    handler.delete_file(remote_path, false).await?;
    handler.delete_file(remote_path_2, false).await?;
    std::fs::remove_file(upload_path)?;
    std::fs::remove_file(download_path)?;

    println!("   Cleanup complete!");

    println!("\n=== All examples completed successfully! ===");

    Ok(())
}
