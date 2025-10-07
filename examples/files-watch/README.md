# files-watch

A filesystem sync daemon for Files.com, demonstrating the files-sdk-rs streaming capabilities.

## Features

### Phase 1 (Complete)
- **Filesystem Watching**: Real-time monitoring of directory changes
- **Upload-only Sync**: Automatically sync local changes to Files.com
- **Progress Tracking**: Visual progress bars for file uploads
- **State Management**: Track synced files to avoid redundant uploads
- **Pattern Ignoring**: Skip files matching glob patterns
- **Incremental Sync**: Only upload changed files

### Phase 2 (Complete)
- **Download Sync**: Sync from Files.com → Local
- **Bidirectional Sync**: Two-way sync with conflict resolution
- **Conflict Resolution**: Newest, largest, or manual resolution strategies
- **Remote Change Detection**: Scan Files.com for changes

## Installation

```bash
cd examples/files-watch
cargo build --release
```

## Configuration

files-watch requires a Files.com API key:

```bash
export FILES_API_KEY=your-api-key-here
```

## Usage

### Initialize a sync configuration

```bash
cargo run -- init /path/to/local/dir --remote /remote/backup
```

With ignore patterns:

```bash
cargo run -- init /path/to/local/dir --remote /remote/backup \
  --ignore "*.tmp" --ignore ".git/*" --ignore "node_modules/*"
```

### Start syncing

```bash
cargo run -- start
```

This will:
1. Perform an initial sync of all files
2. Watch for file changes
3. Automatically upload new/modified files with progress bars

Press Ctrl+C to stop.

### Check status

```bash
cargo run -- status
```

Shows configured watches and their sync status.

### List configurations

```bash
cargo run -- list
```

### One-time sync

Perform a one-time sync without watching:

```bash
# Upload only (default)
cargo run -- sync /path/to/local/dir

# Download only
cargo run -- sync /path/to/local/dir --direction down

# Bidirectional sync with conflict resolution
cargo run -- sync /path/to/local/dir --direction both
```

Full sync (ignore state, sync all files):

```bash
cargo run -- sync /path/to/local/dir --full
```

## Configuration File

Configuration is stored in `~/.files-watch/config.toml`:

```toml
[[watch]]
local_path = "/home/user/documents"
remote_path = "/backup/documents"
direction = "up"
ignore_patterns = ["*.tmp", ".git/*"]

[sync]
check_interval_secs = 60
concurrent_uploads = 5
chunk_size = 65536

[conflict]
resolution = "newest"
```

## State Management

Sync state is tracked in `~/.files-watch/state/*.json` to avoid re-uploading unchanged files.

## Example Session

```bash
# Initialize sync for your documents folder
$ cargo run -- init ~/documents --remote /backup/docs

✓ Initialized sync configuration
  Local:     /home/user/documents
  Remote:    /backup/docs
  Direction: up

Run 'files-watch start' to begin syncing

# Start watching for changes
$ cargo run -- start

Starting files-watch...

  Watching: /home/user/documents
  Remote:   /backup/docs
  Direction: up

Performing initial sync...
✓ 5 files synced

Watching for changes (Ctrl+C to stop)...
→ report.pdf
 ████████████████████████████████████████ 2.5 MB/2.5 MB (00:01)
✓ report.pdf
```

## Limitations

- Single watch config at a time (Phase 3)
- No daemon mode (Phase 3)
- No .filesignore support (Phase 3)

## SDK Features Demonstrated

- **Streaming API**: `FileHandler::upload_stream()`
- **Progress Tracking**: Custom `ProgressCallback` implementation
- **Async File Operations**: Tokio-based file I/O
- **Error Handling**: Comprehensive error context

## Dependencies

- `files-sdk` - Files.com Rust SDK
- `clap` - Command-line parsing
- `tokio` - Async runtime
- `notify` - Filesystem watching
- `indicatif` - Progress bars
- `serde` - Config/state serialization

## Development

Run with debug logging:

```bash
RUST_LOG=debug cargo run -- start -v
```

## Related

- Issue #65: Full specification for files-watch
- Issue #61: Streaming API (completed)
