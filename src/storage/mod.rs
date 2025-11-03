// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! Storage layer for persisting data to files

use std::path::PathBuf;
use crate::Result;

/// Storage manager for data persistence
pub struct Storage {
    data_dir: PathBuf,
}

impl Storage {
    /// Create a new storage instance
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&data_dir)?;
        Ok(Self { data_dir })
    }

    /// Get the data directory path
    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_storage_creation() {
        let dir = tempdir().unwrap();
        let storage = Storage::new(dir.path().to_path_buf()).unwrap();
        assert_eq!(storage.data_dir(), dir.path());
    }
}
