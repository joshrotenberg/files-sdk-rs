//! File ignore pattern matching

use anyhow::Result;
use std::fs;
use std::path::Path;

/// Ignore pattern matcher
#[derive(Debug, Clone)]
pub struct IgnoreMatcher {
    patterns: Vec<String>,
}

impl IgnoreMatcher {
    /// Create a new ignore matcher
    #[allow(dead_code)]
    pub fn new(patterns: Vec<String>) -> Self {
        Self { patterns }
    }

    /// Add a pattern to the matcher
    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    /// Load ignore patterns from a .filesignore file
    pub fn from_file(path: &Path) -> Result<Self> {
        let ignore_file = path.join(".filesignore");

        let patterns = if ignore_file.exists() {
            let contents = fs::read_to_string(&ignore_file)?;
            contents
                .lines()
                .filter(|line| {
                    let trimmed = line.trim();
                    !trimmed.is_empty() && !trimmed.starts_with('#')
                })
                .map(|s| s.to_string())
                .collect()
        } else {
            Vec::new()
        };

        Ok(Self { patterns })
    }

    /// Check if a path should be ignored
    pub fn is_ignored(&self, path: &str) -> bool {
        for pattern in &self.patterns {
            if self.matches_pattern(path, pattern) {
                return true;
            }
        }
        false
    }

    /// Match a path against a pattern
    fn matches_pattern(&self, path: &str, pattern: &str) -> bool {
        // Normalize path separators
        let path = path.replace('\\', "/");
        let pattern = pattern.trim();

        // Handle exact matches
        if path == pattern {
            return true;
        }

        // Handle directory patterns (ending with /)
        if pattern.ends_with('/') {
            let dir_pattern = pattern.trim_end_matches('/');
            if path.starts_with(dir_pattern) {
                return true;
            }
        }

        // Handle wildcard patterns
        if pattern.contains('*') {
            return self.matches_glob(&path, pattern);
        }

        // Handle prefix matches for directories
        if path.starts_with(&format!("{}/", pattern)) {
            return true;
        }

        // Handle suffix matches
        if let Some(suffix) = pattern.strip_prefix('*') {
            if path.ends_with(suffix) {
                return true;
            }
        }

        false
    }

    /// Simple glob matching
    fn matches_glob(&self, path: &str, pattern: &str) -> bool {
        // Convert glob pattern to regex-like matching
        let parts: Vec<&str> = pattern.split('*').collect();

        if parts.is_empty() {
            return false;
        }

        let mut pos = 0;

        // Check first part (must match at start unless pattern starts with *)
        if !pattern.starts_with('*') {
            if !path[pos..].starts_with(parts[0]) {
                return false;
            }
            pos += parts[0].len();
        }

        // Check middle parts
        for part in parts.iter().skip(1).take(parts.len().saturating_sub(2)) {
            if part.is_empty() {
                continue;
            }
            if let Some(idx) = path[pos..].find(part) {
                pos += idx + part.len();
            } else {
                return false;
            }
        }

        // Check last part (must match at end unless pattern ends with *)
        if parts.len() > 1 {
            let last_part = parts[parts.len() - 1];
            if !pattern.ends_with('*') && !last_part.is_empty() {
                return path[pos..].ends_with(last_part);
            } else if !last_part.is_empty() {
                return path[pos..].contains(last_part);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let matcher = IgnoreMatcher::new(vec!["node_modules".to_string()]);
        assert!(matcher.is_ignored("node_modules"));
        assert!(!matcher.is_ignored("src"));
    }

    #[test]
    fn test_directory_pattern() {
        let matcher = IgnoreMatcher::new(vec!["node_modules/".to_string()]);
        assert!(matcher.is_ignored("node_modules"));
        assert!(matcher.is_ignored("node_modules/package"));
        assert!(!matcher.is_ignored("src"));
    }

    #[test]
    fn test_wildcard_suffix() {
        let matcher = IgnoreMatcher::new(vec!["*.log".to_string()]);
        assert!(matcher.is_ignored("app.log"));
        assert!(matcher.is_ignored("error.log"));
        assert!(!matcher.is_ignored("app.txt"));
    }

    #[test]
    fn test_wildcard_pattern() {
        let matcher = IgnoreMatcher::new(vec!["*.tmp".to_string()]);
        assert!(matcher.is_ignored("file.tmp"));
        assert!(matcher.is_ignored("error.tmp"));
        assert!(matcher.is_ignored("dir/file.tmp")); // Suffix match works across paths
        assert!(!matcher.is_ignored("file.txt"));
    }

    #[test]
    fn test_prefix_match() {
        let matcher = IgnoreMatcher::new(vec!["build".to_string()]);
        assert!(matcher.is_ignored("build"));
        assert!(matcher.is_ignored("build/output"));
        assert!(!matcher.is_ignored("src/build.rs"));
    }

    #[test]
    fn test_multiple_patterns() {
        let matcher = IgnoreMatcher::new(vec![
            "*.log".to_string(),
            "node_modules".to_string(),
            "target/".to_string(),
        ]);
        assert!(matcher.is_ignored("app.log"));
        assert!(matcher.is_ignored("node_modules"));
        assert!(matcher.is_ignored("target"));
        assert!(matcher.is_ignored("target/debug"));
        assert!(!matcher.is_ignored("src/main.rs"));
    }
}
