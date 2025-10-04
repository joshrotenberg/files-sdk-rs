# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
