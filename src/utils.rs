//! Utility functions for the Files.com SDK
//!
//! This module provides common utility functions used throughout the SDK,
//! including path encoding, URL construction, and other helpers.

/// Encodes a file path for safe use in URLs
///
/// Files.com paths may contain special characters (spaces, brackets, unicode, etc.)
/// that need to be properly URL-encoded. This function:
/// - Splits the path by `/` to preserve directory structure
/// - Percent-encodes each path segment individually (using %20 for spaces, not +)
/// - Rejoins with `/` separators
///
/// # Arguments
///
/// * `path` - The file or folder path to encode
///
/// # Returns
///
/// A URL-safe encoded path string
///
/// # Examples
///
/// ```
/// use files_sdk::utils::encode_path;
///
/// assert_eq!(encode_path("/my folder/file.txt"), "/my%20folder/file.txt");
/// assert_eq!(encode_path("/data/file[2024].txt"), "/data/file%5B2024%5D.txt");
/// assert_eq!(encode_path("/文档/файл.txt"), "/%E6%96%87%E6%A1%A3/%D1%84%D0%B0%D0%B9%D0%BB.txt");
/// ```
pub fn encode_path(path: &str) -> String {
    // Handle empty or root path
    if path.is_empty() || path == "/" {
        return path.to_string();
    }

    // Split by '/', encode each segment, then rejoin
    let segments: Vec<String> = path
        .split('/')
        .map(|segment| {
            if segment.is_empty() {
                // Preserve empty segments (leading/trailing slashes)
                segment.to_string()
            } else {
                // Percent-encode the segment
                // We use a simple manual approach to ensure %20 for spaces (not +)
                percent_encode(segment)
            }
        })
        .collect();

    segments.join("/")
}

/// Percent-encodes a string for use in URL paths
///
/// Unlike form encoding which uses + for spaces, this uses %20 for spaces
/// and encodes all non-alphanumeric characters except: - _ . ~
///
/// This follows RFC 3986 unreserved characters
fn percent_encode(s: &str) -> String {
    let mut encoded = String::new();

    for byte in s.bytes() {
        match byte {
            // Unreserved characters (RFC 3986)
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            // Everything else gets percent-encoded
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_simple_path() {
        assert_eq!(encode_path("/simple/path.txt"), "/simple/path.txt");
    }

    #[test]
    fn test_encode_path_with_spaces() {
        assert_eq!(
            encode_path("/my folder/my file.txt"),
            "/my%20folder/my%20file.txt"
        );
    }

    #[test]
    fn test_encode_path_with_brackets() {
        assert_eq!(
            encode_path("/data/file[2024].txt"),
            "/data/file%5B2024%5D.txt"
        );
    }

    #[test]
    fn test_encode_path_with_unicode() {
        // Chinese characters
        assert_eq!(
            encode_path("/文档/测试.txt"),
            "/%E6%96%87%E6%A1%A3/%E6%B5%8B%E8%AF%95.txt"
        );

        // Cyrillic characters
        assert_eq!(
            encode_path("/папка/файл.txt"),
            "/%D0%BF%D0%B0%D0%BF%D0%BA%D0%B0/%D1%84%D0%B0%D0%B9%D0%BB.txt"
        );
    }

    #[test]
    fn test_encode_path_with_quotes() {
        assert_eq!(
            encode_path("/\"quoted\"/file.txt"),
            "/%22quoted%22/file.txt"
        );
    }

    #[test]
    fn test_encode_path_with_special_chars() {
        assert_eq!(encode_path("/data/file@#$.txt"), "/data/file%40%23%24.txt");
    }

    #[test]
    fn test_encode_empty_path() {
        assert_eq!(encode_path(""), "");
    }

    #[test]
    fn test_encode_root_path() {
        assert_eq!(encode_path("/"), "/");
    }

    #[test]
    fn test_encode_path_preserves_leading_slash() {
        assert_eq!(encode_path("/folder/file"), "/folder/file");
    }

    #[test]
    fn test_encode_path_without_leading_slash() {
        assert_eq!(encode_path("folder/file"), "folder/file");
    }

    #[test]
    fn test_encode_path_with_trailing_slash() {
        assert_eq!(encode_path("/folder/"), "/folder/");
    }

    #[test]
    fn test_encode_complex_path() {
        // Combination of spaces, brackets, and unicode
        assert_eq!(
            encode_path("/my folder/data [2024]/文档.txt"),
            "/my%20folder/data%20%5B2024%5D/%E6%96%87%E6%A1%A3.txt"
        );
    }
}
