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
    use proptest::prelude::*;

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

    // Property-based tests

    proptest! {
        /// Property: Encoding a path always produces valid ASCII output
        #[test]
        fn prop_encoded_path_is_ascii(path in "(/[^/\0]{0,100}){0,10}") {
            let encoded = encode_path(&path);
            prop_assert!(encoded.is_ascii(), "Encoded path should be ASCII: {}", encoded);
        }

        /// Property: Encoding preserves slash structure
        #[test]
        fn prop_encoding_preserves_slash_count(path in "(/[^/\0]{0,100}){0,10}") {
            let encoded = encode_path(&path);
            let original_slashes = path.matches('/').count();
            let encoded_slashes = encoded.matches('/').count();
            prop_assert_eq!(original_slashes, encoded_slashes,
                "Slash count should be preserved. Original: {}, Encoded: {}", path, encoded);
        }

        /// Property: Encoding is idempotent (encoding an encoded path doesn't change it)
        #[test]
        fn prop_encoding_is_idempotent(path in "[a-zA-Z0-9._~/-]{0,200}") {
            let encoded_once = encode_path(&path);
            let encoded_twice = encode_path(&encoded_once);
            prop_assert_eq!(encoded_once, encoded_twice,
                "Encoding should be idempotent for already-encoded paths");
        }

        /// Property: Empty segments are preserved (leading/trailing slashes)
        #[test]
        fn prop_preserves_leading_slash(path in "/[a-zA-Z0-9._~]{1,50}(/[a-zA-Z0-9._~]{0,50}){0,5}") {
            let encoded = encode_path(&path);
            prop_assert!(encoded.starts_with('/'), "Leading slash should be preserved");
        }

        /// Property: Trailing slashes are preserved
        #[test]
        fn prop_preserves_trailing_slash(path in "[a-zA-Z0-9._~]{1,50}(/[a-zA-Z0-9._~]{0,50}){0,5}/") {
            let encoded = encode_path(&path);
            prop_assert!(encoded.ends_with('/'), "Trailing slash should be preserved");
        }

        /// Property: No double encoding - percent signs in output are only from encoding
        #[test]
        fn prop_no_double_encoding(s in "[^/\0]{1,50}") {
            let path = format!("/{}", s);
            let encoded = encode_path(&path);

            // If there's a % in the encoded output, it should always be followed by exactly 2 hex digits
            let mut chars = encoded.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '%' {
                    let next1 = chars.next();
                    let next2 = chars.next();
                    prop_assert!(next1.is_some() && next2.is_some(),
                        "% should be followed by 2 characters");
                    prop_assert!(next1.unwrap().is_ascii_hexdigit() && next2.unwrap().is_ascii_hexdigit(),
                        "% should be followed by 2 hex digits");
                }
            }
        }

        /// Property: Unreserved characters (A-Za-z0-9-_.~) are never encoded
        #[test]
        fn prop_unreserved_never_encoded(s in "[A-Za-z0-9._~-]+") {
            let encoded = encode_path(&s);
            prop_assert_eq!(&encoded, &s, "Unreserved characters should not be encoded");
        }

        /// Property: Spaces are always encoded as %20
        #[test]
        fn prop_spaces_encoded_as_percent20(s in "[a-z ]{1,50}") {
            let path = format!("/{}", s);
            let encoded = encode_path(&path);

            if s.contains(' ') {
                prop_assert!(encoded.contains("%20"), "Spaces should be encoded as %20");
                prop_assert!(!encoded.contains('+'), "Spaces should not be encoded as +");
            }
        }

        /// Property: Very long paths don't panic
        #[test]
        fn prop_handles_long_paths(path in "(/[a-zA-Z0-9]{0,500}){0,20}") {
            let _ = encode_path(&path); // Should not panic
        }

        /// Property: Unicode characters are percent-encoded
        #[test]
        fn prop_unicode_is_encoded(s in "[\\u{0080}-\\u{FFFF}]{1,20}") {
            let path = format!("/{}", s);
            let encoded = encode_path(&path);

            // Unicode should be encoded (will contain %)
            if !s.is_ascii() {
                prop_assert!(encoded.contains('%'),
                    "Non-ASCII unicode should be percent-encoded: {} -> {}", s, encoded);
            }
        }

        /// Property: Root path is unchanged
        #[test]
        fn prop_root_path_unchanged(_unit in prop::bool::ANY) {
            prop_assert_eq!(encode_path("/"), "/");
        }

        /// Property: Empty path is unchanged
        #[test]
        fn prop_empty_path_unchanged(_unit in prop::bool::ANY) {
            prop_assert_eq!(encode_path(""), "");
        }
    }
}
