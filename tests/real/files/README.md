# Files Module Comprehensive Test Suite

Comprehensive real API integration tests for the files/ module covering all 7 handlers.

## Test Coverage

### files.rs (FileHandler) - 17 tests
**Basic Operations (files.rs):**
- ✅ Upload and download
- ✅ File operations (copy, move, delete)
- ✅ Large file upload (1MB)
- ✅ Folder operations

**Comprehensive Tests (files_comprehensive.rs):**
- ✅ Delete with recursive flag
- ✅ File not found error handling
- ✅ Special characters in filenames (spaces, dashes, underscores, dots)
- ✅ Update file metadata
- ✅ Copy to different folder
- ✅ Move/rename files
- ✅ File conflicts on copy
- ✅ Deep nested folder paths

### folders.rs (FolderHandler) - 8 tests
- ✅ List with pagination (cursor support)
- ✅ Search functionality with filters
- ✅ Create with mkdir_parents
- ✅ Delete recursive
- ✅ List empty folder
- ✅ Folder not found error
- ✅ Folder name conflicts
- ✅ Special characters in folder names

### file_actions.rs (FileActionHandler) - 9 tests
- ✅ begin_upload for small files
- ✅ begin_upload for large files (multipart detection)
- ✅ begin_upload without mkdir_parents (error case)
- ✅ Copy via file_action
- ✅ Move via file_action
- ✅ Copy to existing destination (conflict)
- ✅ Metadata retrieval
- ✅ begin_upload with custom etag

### file_comments.rs (FileCommentHandler + Reactions) - 8 tests
- ✅ Full comment workflow (create → list → update → delete)
- ✅ Comment reaction workflow (create comment → add reaction → delete reaction)
- ✅ Multiple comments on single file
- ✅ Comment on nonexistent file (error case)
- ✅ Empty comment body validation
- ✅ Update nonexistent comment (error case)
- ✅ Delete nonexistent comment (error case)

## Total: 42 comprehensive tests

## Running Tests

All tests require the `FILES_API_KEY` environment variable and the `integration-tests` feature flag:

```bash
# Run all files/ module tests
FILES_API_KEY=your_key cargo test --test real --features integration-tests files::

# Run specific test file
FILES_API_KEY=your_key cargo test --test real --features integration-tests files::file_comments::

# Run specific test
FILES_API_KEY=your_key cargo test --test real --features integration-tests test_file_comment_workflow
```

## Test Organization

- `files.rs` - Original basic tests
- `files_comprehensive.rs` - Edge cases and error scenarios for FileHandler
- `folders.rs` - Complete FolderHandler coverage
- `file_actions.rs` - FileActionHandler operations
- `file_comments.rs` - Comments and reactions workflows

## Coverage Status

| Handler | Tests | Status |
|---------|-------|--------|
| files.rs | 17 | ✅ Comprehensive |
| folders.rs | 8 | ✅ Comprehensive |
| file_actions.rs | 9 | ✅ Comprehensive |
| file_comments.rs | 8 | ✅ Comprehensive |
| file_comment_reactions.rs | Integrated | ✅ Covered in file_comments |
| file_migrations.rs | 0 | ⬜ Not yet covered |
| file_migration_logs.rs | 0 | ⬜ Not yet covered |

**Total Coverage: 5/7 handlers (71%)**

## Next Steps

- Add tests for file_migrations.rs (get migration status)
- Add tests for file_migration_logs.rs (list logs)
- Consider adding performance tests for large file uploads (>10MB)
- Add tests for quota/rate limiting scenarios
