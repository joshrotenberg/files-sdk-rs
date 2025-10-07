# Contributing to files-sdk-rs

Thank you for your interest in contributing to the Files.com Rust SDK! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Standards](#code-standards)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- **Rust**: Latest stable version (we follow Rust 2024 edition)
- **Files.com API Key**: Required for integration tests
  - Sign up at https://files.com for a free account
  - Generate an API key from Account Settings → API Keys
  - Set as environment variable: `export FILES_API_KEY=your-api-key`

### Development Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/joshrotenberg/files-sdk-rs.git
   cd files-sdk-rs
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run tests**:
   ```bash
   # Unit tests only (no API key required)
   cargo test --lib
   
   # All tests including integration tests (requires API key)
   FILES_API_KEY=your-key cargo test --all-features
   ```

## Code Standards

### Rust Edition and Idioms

- **Edition**: Rust 2024
- Follow idiomatic Rust patterns and conventions
- Use `async/await` for asynchronous operations
- Prefer `Result` over panicking
- Use `thiserror` for library errors (not `anyhow`)

### Code Formatting

All code must be formatted with `rustfmt`:

```bash
cargo fmt --all -- --check
```

### Linting

All code must pass clippy without warnings:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Documentation

- **All public APIs** must have doc comments
- Include examples in doc comments where appropriate
- Use `///` for item documentation, `//!` for module documentation
- Examples in doc comments should compile and run as doc tests

Example:
```rust
/// Uploads a file to Files.com
///
/// # Arguments
///
/// * `path` - The remote path where the file will be stored
/// * `data` - The file contents as bytes
///
/// # Examples
///
/// ```no_run
/// use files_sdk::FilesClient;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = FilesClient::builder()
///     .api_key("your-api-key")
///     .build()?;
///     
/// client.upload("/path/to/file.txt", b"Hello, world!").await?;
/// # Ok(())
/// # }
/// ```
pub async fn upload(&self, path: &str, data: &[u8]) -> Result<()> {
    // implementation
}
```

### Testing Requirements

- **Test Coverage**: Aim for minimum 70% coverage for new code
- **Unit Tests**: Required for all business logic
- **Integration Tests**: Required for API interactions
- **Doc Tests**: All examples in documentation must compile

## Testing

### Running Tests

```bash
# Unit tests only (fast, no API key needed)
cargo test --lib

# Integration tests (requires FILES_API_KEY)
FILES_API_KEY=your-key cargo test --test '*'

# All tests
FILES_API_KEY=your-key cargo test --all-features

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture
```

### Writing Tests

#### Unit Tests

Place unit tests in the same file as the code being tested:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        assert_eq!(2 + 2, 4);
    }
}
```

#### Integration Tests

Place integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
#[tokio::test]
#[ignore] // Run only with --ignored or --all-features
async fn test_file_upload() {
    let api_key = std::env::var("FILES_API_KEY")
        .expect("FILES_API_KEY must be set");
    
    let client = FilesClient::builder()
        .api_key(api_key)
        .build()
        .unwrap();
    
    // Test implementation
}
```

## Submitting Changes

### Git Workflow

We follow a **feature branch workflow**:

1. **ALWAYS** create a feature branch before making changes:
   ```bash
   git checkout -b feat/your-feature-name
   # or
   git checkout -b fix/bug-description
   ```

2. **NEVER** commit directly to `main` branch

3. Branch naming conventions:
   - `feat/` - New features
   - `fix/` - Bug fixes
   - `docs/` - Documentation changes
   - `refactor/` - Code refactoring
   - `test/` - Test improvements
   - `chore/` - Maintenance tasks

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>: <description>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:
```bash
feat: add webhook signature verification

fix: handle empty file uploads correctly

docs: update README with streaming examples

test: add integration tests for automation module
```

**Important**:
- NO emojis in commit messages
- Reference issues: `Closes #123` or `Fixes #456`
- Keep first line under 72 characters

### Pull Request Process

1. **Before submitting**:
   ```bash
   # Format code
   cargo fmt --all
   
   # Run clippy
   cargo clippy --all-targets --all-features -- -D warnings
   
   # Run tests
   cargo test --lib --all-features
   cargo test --test '*' --all-features
   ```

2. **Create Pull Request**:
   - Use a clear, descriptive title
   - Fill out the PR template (if provided)
   - Reference related issues
   - Describe what changed and why
   - Include examples if adding new features

3. **PR Review**:
   - Address review feedback promptly
   - Keep PR scope focused (one feature/fix per PR)
   - Maintain a clean commit history

4. **Merging**:
   - DO NOT merge your own PRs without approval
   - Wait for CI checks to pass
   - Squash commits if requested

### Pre-commit Checklist

Before pushing your changes:

- [ ] Code is formatted (`cargo fmt --all`)
- [ ] Clippy passes with no warnings (`cargo clippy --all-targets --all-features -- -D warnings`)
- [ ] All tests pass (`cargo test --all-features`)
- [ ] New code has tests
- [ ] Public APIs have documentation
- [ ] CHANGELOG.md updated (if applicable)
- [ ] Commit messages follow conventional commits format
- [ ] No debug code or commented-out code
- [ ] Branch is up to date with `main`

## Release Process

Releases are managed by maintainers using [release-plz](https://release-plz.ieni.dev/):

1. Changes are merged to `main`
2. `release-plz` creates a PR with version bump and CHANGELOG updates
3. Maintainer reviews and merges the release PR
4. CI automatically publishes to crates.io

**Version Bumping**:
- Breaking changes: Major version (1.0.0 → 2.0.0)
- New features: Minor version (0.3.0 → 0.4.0)
- Bug fixes: Patch version (0.3.1 → 0.3.2)

Note: Pre-1.0 versions may include breaking changes in minor releases.

## Project Structure

```
files-sdk-rs/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── client.rs           # HTTP client and builder
│   ├── error.rs            # Error types
│   ├── progress.rs         # Progress tracking
│   ├── files/              # File operation handlers
│   ├── users/              # User management handlers
│   ├── sharing/            # Sharing handlers
│   ├── automation/         # Automation handlers
│   └── ...                 # Other resource modules
├── tests/                  # Integration tests
├── examples/               # Example applications
│   └── files-watch/        # Filesystem sync daemon example
├── CLAUDE.md               # Project context and decisions
├── CHANGELOG.md            # Version history
└── CONTRIBUTING.md         # This file
```

## Getting Help

- **Questions**: Open a GitHub Discussion
- **Bugs**: Open an issue with detailed reproduction steps
- **Features**: Open an issue describing the use case
- **Security**: Email maintainers directly (see SECURITY.md if available)

## Additional Resources

- [Files.com API Documentation](https://developers.files.com/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see LICENSE file).

## Questions?

If you have questions about contributing, feel free to:
- Open a GitHub Discussion
- Open an issue
- Contact the maintainers

Thank you for contributing to files-sdk-rs!
