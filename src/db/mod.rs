// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! Database module for SQLite operations

pub mod models;
pub mod person_repo;
pub mod project_repo;
pub mod schema;

pub use models::{Milestone, MilestoneNote, Person, Project, ProjectNote, ProjectStakeholder, StakeholderNote};
pub use person_repo::PersonRepository;
pub use project_repo::ProjectRepository;

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

/// Open or create a database connection
pub fn open_database<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let path = path.as_ref();

    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create database directory: {}", parent.display()))?;
    }

    let conn = Connection::open(path)
        .with_context(|| format!("Failed to open database: {}", path.display()))?;

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Initialize schema
    schema::initialize_schema(&conn)?;

    // Apply migrations
    schema::apply_migrations(&conn)?;

    log::info!("Database opened: {}", path.display());
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_open_database() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let conn = open_database(&db_path).unwrap();

        // Verify foreign keys are enabled
        let fk_enabled: i32 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(fk_enabled, 1);

        // Verify schema exists and migrations applied
        let version = schema::get_schema_version(&conn).unwrap();
        assert_eq!(version, 3); // Current version after all migrations
    }
}
