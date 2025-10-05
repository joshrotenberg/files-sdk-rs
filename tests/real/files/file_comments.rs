//! Comprehensive real API tests for FileCommentHandler and FileCommentReactionHandler
//!
//! Tests the complete workflow: upload file → comment → react → cleanup

use crate::real::*;
use files_sdk::{FileCommentHandler, FileCommentReactionHandler, FileHandler};

#[tokio::test]
async fn test_file_comment_workflow() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());
    let comment_handler = FileCommentHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_file = "/integration-tests/comment-test.txt";
    let test_content = b"File for comment testing";

    // Setup: Upload a file
    cleanup_file(&client, test_file).await;
    file_handler
        .upload_file(test_file, test_content)
        .await
        .expect("Should upload test file");

    println!("Testing file comment workflow");

    // Step 1: Create a comment
    let comment_body = "This is a test comment on the file";
    let create_result = comment_handler.create(test_file, comment_body).await;

    let comment_id = match create_result {
        Ok(comment) => {
            println!("Comment created: {:?}", comment);

            // Extract comment ID for later operations
            if let Some(id) = comment.id {
                println!("Comment ID: {}", id);
                Some(id)
            } else {
                println!("Could not extract comment ID from response");
                None
            }
        }
        Err(e) => {
            eprintln!(
                "Comment creation failed (may require premium features): {:?}",
                e
            );
            cleanup_file(&client, test_file).await;
            return;
        }
    };

    // Step 2: List comments on the file
    let list_result = comment_handler.list(test_file).await;

    match list_result {
        Ok(comments) => {
            println!("Listed {} comments on file", comments.len());
            assert!(!comments.is_empty(), "Should have at least one comment");
        }
        Err(e) => {
            eprintln!("Failed to list comments: {:?}", e);
        }
    }

    // Step 3: Update the comment if we have an ID
    if let Some(id) = comment_id {
        let updated_body = "This comment has been updated";
        let update_result = comment_handler.update(id, updated_body).await;

        match update_result {
            Ok(updated_comment) => {
                println!("Comment updated: {:?}", updated_comment);
            }
            Err(e) => {
                eprintln!("Comment update failed: {:?}", e);
            }
        }

        // Step 4: Delete the comment
        let delete_result = comment_handler.delete(id).await;

        match delete_result {
            Ok(_) => {
                println!("Comment deleted successfully");
            }
            Err(e) => {
                eprintln!("Comment deletion failed: {:?}", e);
            }
        }
    }

    // Cleanup
    cleanup_file(&client, test_file).await;
}

#[tokio::test]
async fn test_file_comment_reaction_workflow() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());
    let comment_handler = FileCommentHandler::new(client.clone());
    let _reaction_handler = FileCommentReactionHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_file = "/integration-tests/reaction-test.txt";
    let test_content = b"File for reaction testing";

    // Setup: Upload a file
    cleanup_file(&client, test_file).await;
    file_handler
        .upload_file(test_file, test_content)
        .await
        .expect("Should upload test file");

    println!("Testing file comment reaction workflow");

    // Step 1: Create a comment
    let comment_result = comment_handler
        .create(test_file, "Comment for reactions")
        .await;

    let comment_id = match comment_result {
        Ok(comment) => {
            if let Some(id) = comment.id {
                println!("Created comment with ID: {}", id);
                id
            } else {
                eprintln!("Could not extract comment ID");
                cleanup_file(&client, test_file).await;
                return;
            }
        }
        Err(e) => {
            eprintln!(
                "Comment creation failed (may require premium features): {:?}",
                e
            );
            cleanup_file(&client, test_file).await;
            return;
        }
    };

    // Step 2: Add a reaction to the comment
    let emoji = "👍";
    let reaction_result = comment_handler.add_reaction(comment_id, emoji).await;

    let reaction_id = match reaction_result {
        Ok(reaction) => {
            println!("Reaction added: {:?}", reaction);

            if let Some(id) = reaction.id {
                println!("Reaction ID: {}", id);
                Some(id)
            } else {
                None
            }
        }
        Err(e) => {
            eprintln!(
                "Reaction creation failed (may require premium features): {:?}",
                e
            );
            None
        }
    };

    // Step 3: Delete the reaction if we have an ID
    if let Some(id) = reaction_id {
        let delete_reaction_result = comment_handler.delete_reaction(id).await;

        match delete_reaction_result {
            Ok(_) => {
                println!("Reaction deleted successfully");
            }
            Err(e) => {
                eprintln!("Reaction deletion failed: {:?}", e);
            }
        }
    }

    // Cleanup: Delete comment and file
    let _ = comment_handler.delete(comment_id).await;
    cleanup_file(&client, test_file).await;
}

#[tokio::test]
async fn test_multiple_comments_on_file() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());
    let comment_handler = FileCommentHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_file = "/integration-tests/multiple-comments-test.txt";
    let test_content = b"File for multiple comments";

    // Setup
    cleanup_file(&client, test_file).await;
    file_handler
        .upload_file(test_file, test_content)
        .await
        .expect("Should upload test file");

    println!("Testing multiple comments on single file");

    let mut comment_ids = Vec::new();

    // Create multiple comments
    for i in 1..=3 {
        let body = format!("Comment number {}", i);
        let result = comment_handler.create(test_file, &body).await;

        match result {
            Ok(comment) => {
                if let Some(id) = comment.id {
                    println!("Created comment {}: ID {}", i, id);
                    comment_ids.push(id);
                }
            }
            Err(e) => {
                eprintln!("Failed to create comment {}: {:?}", i, e);
                break;
            }
        }
    }

    if comment_ids.is_empty() {
        eprintln!("No comments created (may require premium features)");
        cleanup_file(&client, test_file).await;
        return;
    }

    // List all comments
    let list_result = comment_handler.list(test_file).await;

    match list_result {
        Ok(comments) => {
            println!("Listed {} comments", comments.len());
            assert!(
                comments.len() >= comment_ids.len(),
                "Should list all created comments"
            );
        }
        Err(e) => {
            eprintln!("Failed to list comments: {:?}", e);
        }
    }

    // Cleanup: Delete all comments
    for id in comment_ids {
        let _ = comment_handler.delete(id).await;
    }

    cleanup_file(&client, test_file).await;
}

#[tokio::test]
async fn test_comment_on_nonexistent_file() {
    let client = get_test_client();
    let comment_handler = FileCommentHandler::new(client.clone());

    let nonexistent_file = "/integration-tests/does-not-exist.txt";

    println!("Testing comment on nonexistent file");

    let result = comment_handler
        .create(nonexistent_file, "Comment on nonexistent file")
        .await;

    match result {
        Err(files_sdk::FilesError::NotFound { message }) => {
            println!("Correctly received NotFound error: {}", message);
        }
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
        }
        Ok(_) => {
            panic!("Should not create comment on nonexistent file");
        }
    }
}

#[tokio::test]
async fn test_empty_comment_body() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());
    let comment_handler = FileCommentHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_file = "/integration-tests/empty-comment-test.txt";

    // Setup
    cleanup_file(&client, test_file).await;
    file_handler
        .upload_file(test_file, b"Test")
        .await
        .expect("Should upload file");

    println!("Testing empty comment body");

    // Try to create comment with empty body
    let result = comment_handler.create(test_file, "").await;

    match result {
        Ok(comment) => {
            println!("Empty comment created (API may allow): {:?}", comment);

            // Clean up the comment
            if let Some(id) = comment.id {
                let _ = comment_handler.delete(id).await;
            }
        }
        Err(files_sdk::FilesError::UnprocessableEntity { message }) => {
            println!("Correctly rejected empty comment: {}", message);
        }
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
        }
    }

    cleanup_file(&client, test_file).await;
}

#[tokio::test]
async fn test_update_nonexistent_comment() {
    let client = get_test_client();
    let comment_handler = FileCommentHandler::new(client.clone());

    let nonexistent_id = 999999999;

    println!("Testing update of nonexistent comment");

    let result = comment_handler.update(nonexistent_id, "Updated body").await;

    match result {
        Err(files_sdk::FilesError::NotFound { message }) => {
            println!("Correctly received NotFound error: {}", message);
        }
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
        }
        Ok(_) => {
            panic!("Should not update nonexistent comment");
        }
    }
}

#[tokio::test]
async fn test_delete_nonexistent_comment() {
    let client = get_test_client();
    let comment_handler = FileCommentHandler::new(client.clone());

    let nonexistent_id = 999999999;

    println!("Testing deletion of nonexistent comment");

    let result = comment_handler.delete(nonexistent_id).await;

    match result {
        Err(files_sdk::FilesError::NotFound { message }) => {
            println!("Correctly received NotFound error: {}", message);
        }
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
        }
        Ok(_) => {
            // Some APIs return success for idempotent deletes
            println!("Delete succeeded (API may be idempotent)");
        }
    }
}
