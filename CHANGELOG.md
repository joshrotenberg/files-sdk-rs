# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1](https://github.com/joshrotenberg/files-sdk-rs/compare/v0.3.0...v0.3.1) - 2025-10-06

### Other

- simplify README, remove marketing language ([#57](https://github.com/joshrotenberg/files-sdk-rs/pull/57))
- improve README with Files.com context and community disclaimer ([#56](https://github.com/joshrotenberg/files-sdk-rs/pull/56))
- add comprehensive integration tests for sharing, automation, and admin modules ([#55](https://github.com/joshrotenberg/files-sdk-rs/pull/55))

## [0.3.0](https://github.com/joshrotenberg/files-sdk-rs/compare/v0.2.0...v0.3.0) - 2025-10-06

### Added

- add recursive directory upload with progress callback ([#52](https://github.com/joshrotenberg/files-sdk-rs/pull/52))
- add auto-paginating stream iterators for list operations ([#50](https://github.com/joshrotenberg/files-sdk-rs/pull/50))
- add methods to download actual file content ([#48](https://github.com/joshrotenberg/files-sdk-rs/pull/48))
- add URL encoding for file paths with special characters ([#47](https://github.com/joshrotenberg/files-sdk-rs/pull/47))

### Other

- extract error types to dedicated module with contextual fields ([#49](https://github.com/joshrotenberg/files-sdk-rs/pull/49))
- move a few things around ([#36](https://github.com/joshrotenberg/files-sdk-rs/pull/36))
- replace manual cache steps with Swatinem/rust-cache ([#35](https://github.com/joshrotenberg/files-sdk-rs/pull/35))
- reorganize handlers to match Files.com documentation categories ([#33](https://github.com/joshrotenberg/files-sdk-rs/pull/33))

## [0.2.0](https://github.com/joshrotenberg/files-sdk-rs/compare/v0.1.1...v0.2.0) - 2025-10-06

### Added

- add UserSftpClientUse handler to achieve 100% API coverage ([#30](https://github.com/joshrotenberg/files-sdk-rs/pull/30))

### Fixed

- adjust integration test assertions for real API behavior ([#29](https://github.com/joshrotenberg/files-sdk-rs/pull/29))

### Other

- add comprehensive rustdoc with examples to key handlers ([#31](https://github.com/joshrotenberg/files-sdk-rs/pull/31))

## [0.1.0] - 2025-01-XX

### Added
- Complete API coverage: 288 endpoints across 90+ handlers
- Domain-organized modules (files, users, sharing, automation, admin, logs, messages, storage, security, as2, advanced)
- Comprehensive file operations (upload, download, copy, move, delete, comments)
- User management (users, groups, API keys, sessions, permissions)
- Sharing features (bundles, requests, inbox uploads)
- Automation (automations, behaviors, webhooks, remote servers)
- Admin features (site settings, history, invoices, payments)
- Logging (API logs, SFTP/FTP logs, automation logs, sync logs)
- Storage management (projects, snapshots, locks)
- Security (GPG keys, SFTP host keys, clickwraps)
- AS2 protocol support (stations, partners, messages)
- Builder pattern for client configuration
- Comprehensive error handling (14 error types)
- Cursor-based pagination support
- Optional tracing feature for HTTP-level debugging
- 56 integration tests across core modules
- 177 mock tests for unit coverage
- 52 unit tests for handlers

### Changed
- Organized handlers into domain modules for better discoverability
- Improved error types with detailed HTTP status code mapping

### Fixed
- Clippy warnings in test files
- Module inception issues in test organization
- Client borrow/move issues in integration tests

[Unreleased]: https://github.com/joshrotenberg/files-idk-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/joshrotenberg/files-idk-rs/releases/tag/v0.1.0
