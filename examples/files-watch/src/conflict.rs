//! Conflict resolution for bidirectional sync

use anyhow::Result;
use chrono::{DateTime, Utc};
use std::str::FromStr;

/// Represents a file that exists in both local and remote locations
#[derive(Debug, Clone)]
pub struct FileConflict {
    #[allow(dead_code)]
    pub path: String,
    pub local_size: u64,
    pub local_mtime: DateTime<Utc>,
    pub remote_size: i64,
    pub remote_mtime: DateTime<Utc>,
}

/// Resolution strategy for conflicts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictResolution {
    /// Use the file with the newest modification time
    Newest,
    /// Use the file with the largest size
    Largest,
    /// Skip conflicted files (require manual resolution)
    Manual,
}

impl FromStr for ConflictResolution {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "newest" => Ok(Self::Newest),
            "largest" => Ok(Self::Largest),
            "manual" => Ok(Self::Manual),
            _ => anyhow::bail!("Invalid conflict resolution: {}", s),
        }
    }
}

/// Determines which version should win in a conflict
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictWinner {
    Local,
    Remote,
    Skip,
}

impl FileConflict {
    /// Resolve the conflict based on the given strategy
    pub fn resolve(&self, strategy: ConflictResolution) -> ConflictWinner {
        match strategy {
            ConflictResolution::Newest => {
                if self.local_mtime > self.remote_mtime {
                    ConflictWinner::Local
                } else if self.remote_mtime > self.local_mtime {
                    ConflictWinner::Remote
                } else {
                    // Same timestamp - use size as tiebreaker
                    if self.local_size > self.remote_size as u64 {
                        ConflictWinner::Local
                    } else {
                        ConflictWinner::Remote
                    }
                }
            }
            ConflictResolution::Largest => {
                if self.local_size > self.remote_size as u64 {
                    ConflictWinner::Local
                } else if (self.remote_size as u64) > self.local_size {
                    ConflictWinner::Remote
                } else {
                    // Same size - use time as tiebreaker
                    if self.local_mtime > self.remote_mtime {
                        ConflictWinner::Local
                    } else {
                        ConflictWinner::Remote
                    }
                }
            }
            ConflictResolution::Manual => ConflictWinner::Skip,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_newest_local_wins() {
        let conflict = FileConflict {
            path: "test.txt".to_string(),
            local_size: 100,
            local_mtime: Utc::now(),
            remote_size: 100,
            remote_mtime: Utc::now() - chrono::Duration::hours(1),
        };

        assert_eq!(
            conflict.resolve(ConflictResolution::Newest),
            ConflictWinner::Local
        );
    }

    #[test]
    fn test_resolve_newest_remote_wins() {
        let conflict = FileConflict {
            path: "test.txt".to_string(),
            local_size: 100,
            local_mtime: Utc::now() - chrono::Duration::hours(1),
            remote_size: 100,
            remote_mtime: Utc::now(),
        };

        assert_eq!(
            conflict.resolve(ConflictResolution::Newest),
            ConflictWinner::Remote
        );
    }

    #[test]
    fn test_resolve_largest_local_wins() {
        let now = Utc::now();
        let conflict = FileConflict {
            path: "test.txt".to_string(),
            local_size: 200,
            local_mtime: now,
            remote_size: 100,
            remote_mtime: now,
        };

        assert_eq!(
            conflict.resolve(ConflictResolution::Largest),
            ConflictWinner::Local
        );
    }

    #[test]
    fn test_resolve_manual_always_skips() {
        let now = Utc::now();
        let conflict = FileConflict {
            path: "test.txt".to_string(),
            local_size: 100,
            local_mtime: now,
            remote_size: 100,
            remote_mtime: now,
        };

        assert_eq!(
            conflict.resolve(ConflictResolution::Manual),
            ConflictWinner::Skip
        );
    }

    #[test]
    fn test_resolution_from_str() {
        assert_eq!(
            ConflictResolution::from_str("newest").unwrap(),
            ConflictResolution::Newest
        );
        assert_eq!(
            ConflictResolution::from_str("largest").unwrap(),
            ConflictResolution::Largest
        );
        assert_eq!(
            ConflictResolution::from_str("manual").unwrap(),
            ConflictResolution::Manual
        );
        assert!(ConflictResolution::from_str("invalid").is_err());
    }
}
