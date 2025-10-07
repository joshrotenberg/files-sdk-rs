# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/joshrotenberg/files-sdk-rs/compare/v0.3.1...v0.4.0) - 2025-10-07

### Added

- add client-side rate limiting with governor ([#85](https://github.com/joshrotenberg/files-sdk-rs/pull/85))
- add retry logic with exponential backoff and improved JSON errors ([#84](https://github.com/joshrotenberg/files-sdk-rs/pull/84))
- add prelude module for common imports ([#83](https://github.com/joshrotenberg/files-sdk-rs/pull/83))
- add files-watch example - Phase 1 (basic upload sync) ([#66](https://github.com/joshrotenberg/files-sdk-rs/pull/66))
- polish streaming API with documentation and optimizations (Phase 3) ([#64](https://github.com/joshrotenberg/files-sdk-rs/pull/64))
- add progress callbacks for streaming operations (Phase 2) ([#63](https://github.com/joshrotenberg/files-sdk-rs/pull/63))
- add streaming upload/download methods (Phase 1) ([#62](https://github.com/joshrotenberg/files-sdk-rs/pull/62))

### Fixed

- add Content-Length header for S3 uploads (replaces #67) ([#68](https://github.com/joshrotenberg/files-sdk-rs/pull/68))
- handle empty file uploads in upload_stream ([#67](https://github.com/joshrotenberg/files-sdk-rs/pull/67))

### Other

- fix repository URL in Cargo.toml ([#93](https://github.com/joshrotenberg/files-sdk-rs/pull/93))
- migrate to tower-based middleware architecture ([#92](https://github.com/joshrotenberg/files-sdk-rs/pull/92))
- add API stability guarantees and path to 1.0 ([#90](https://github.com/joshrotenberg/files-sdk-rs/pull/90))
- update governor from 0.8 to 0.10 ([#88](https://github.com/joshrotenberg/files-sdk-rs/pull/88))
- add property-based tests for path encoding ([#89](https://github.com/joshrotenberg/files-sdk-rs/pull/89))
- add comprehensive testing examples and documentation ([#87](https://github.com/joshrotenberg/files-sdk-rs/pull/87))
- add CHANGELOG.md and CONTRIBUTING.md ([#82](https://github.com/joshrotenberg/files-sdk-rs/pull/82))
- add comprehensive integration tests for storage, security, logs, messages, AS2, developers, and integrations ([#59](https://github.com/joshrotenberg/files-sdk-rs/pull/59))

### Added
- CHANGELOG.md following Keep a Changelog format
- CONTRIBUTING.md with development guidelines

## [0.3.1] - 2024-12-XX

### Fixed
- Empty file uploads now work correctly with Content-Length header
- S3 upload compatibility improved

### Documentation
- Simplified README, removed marketing language
- Added Files.com context and community disclaimer

## [0.3.0] - 2024-11-XX

### Added
- **100% API Coverage**: All 90 resources and 288 endpoints implemented
- **Streaming API**: 
  - `upload_stream()` and `download_stream()` methods with progress callbacks
  - Memory-efficient large file handling
  - Custom progress tracking via `ProgressCallback` trait
- **Enhanced Pagination**:
  - Auto-paginating stream iterators for list operations
  - Three pagination approaches: manual, auto, and streaming
- **Recursive Operations**:
  - Recursive directory upload with progress tracking
  - Automatic path handling
- **Error Handling**:
  - Dedicated error module with contextual fields
  - `is_retryable()` method for transient errors
  - Error helpers for common cases
- **HTTP Headers**:
  - User-Agent header: "Files.com Rust SDK {version}"
  - Content-Type header for API requests
- **Integration Tests**:
  - Comprehensive tests for sharing, automation, admin modules
  - Storage, security, logs, messages modules
  - AS2, developers, integrations modules
  - Tests against real Files.com API

### Changed
- Refactored error types to separate module for better organization
- Improved file download methods to return actual content

## [0.2.0] - 2024-10-XX

### Added
- Additional resource handlers (expanded coverage)
- More comprehensive API endpoint support

## [0.1.1] - 2024-09-XX

### Fixed
- Bug fixes and stability improvements

## [0.1.0] - 2024-09-XX

### Added
- Initial release
- Core client with builder pattern
- Basic file operations (upload, download, list)
- Authentication via API key
- Basic error handling
- Fundamental resource handlers:
  - Files and folders
  - Users and groups
  - Permissions
  - Sessions and API keys

[Unreleased]: https://github.com/joshrotenberg/files-sdk-rs/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/joshrotenberg/files-sdk-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/joshrotenberg/files-sdk-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/joshrotenberg/files-sdk-rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/joshrotenberg/files-sdk-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/joshrotenberg/files-sdk-rs/releases/tag/v0.1.0
