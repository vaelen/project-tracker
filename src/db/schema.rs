// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

use anyhow::Result;
use rusqlite::Connection;

/// Initialize the database schema
pub fn initialize_schema(conn: &Connection) -> Result<()> {
    // Create people table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS people (
            email TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            team TEXT,
            manager TEXT,
            notes TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (manager) REFERENCES people(email)
        )",
        [],
    )?;

    // Create index on people names for autocomplete
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_people_name ON people(name)",
        [],
    )?;

    // Create projects table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            type TEXT NOT NULL DEFAULT 'Personal',
            requirements_owner TEXT,
            technical_lead TEXT,
            manager TEXT,
            due_date TEXT,
            jira_initiative TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (requirements_owner) REFERENCES people(email),
            FOREIGN KEY (technical_lead) REFERENCES people(email),
            FOREIGN KEY (manager) REFERENCES people(email)
        )",
        [],
    )?;

    // Create index on project names
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_projects_name ON projects(name)",
        [],
    )?;

    // Create milestones table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS milestones (
            id TEXT PRIMARY KEY NOT NULL,
            project_id TEXT NOT NULL,
            number INTEGER NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            technical_lead TEXT,
            design_doc_url TEXT,
            due_date TEXT,
            jira_epic TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (technical_lead) REFERENCES people(email),
            UNIQUE(project_id, number)
        )",
        [],
    )?;

    // Create index on milestone due dates
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_milestones_due_date ON milestones(due_date)",
        [],
    )?;

    // Create project_stakeholders junction table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS project_stakeholders (
            project_id TEXT NOT NULL,
            stakeholder_email TEXT NOT NULL,
            role TEXT,
            created_at TEXT NOT NULL,
            PRIMARY KEY (project_id, stakeholder_email),
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
            FOREIGN KEY (stakeholder_email) REFERENCES people(email)
        )",
        [],
    )?;

    // Create project_notes table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS project_notes (
            id TEXT PRIMARY KEY NOT NULL,
            project_id TEXT NOT NULL,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create milestone_notes table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS milestone_notes (
            id TEXT PRIMARY KEY NOT NULL,
            milestone_id TEXT NOT NULL,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (milestone_id) REFERENCES milestones(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create stakeholder_notes table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stakeholder_notes (
            id TEXT PRIMARY KEY NOT NULL,
            project_id TEXT NOT NULL,
            stakeholder_email TEXT NOT NULL,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id, stakeholder_email) REFERENCES project_stakeholders(project_id, stakeholder_email) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create schema_version table for migrations
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY NOT NULL,
            applied_at TEXT NOT NULL
        )",
        [],
    )?;

    // Insert initial schema version if not exists
    conn.execute(
        "INSERT OR IGNORE INTO schema_version (version, applied_at)
         VALUES (1, datetime('now'))",
        [],
    )?;

    log::info!("Database schema initialized");
    Ok(())
}

/// Get the current schema version
pub fn get_schema_version(conn: &Connection) -> Result<i32> {
    let version: i32 = conn.query_row(
        "SELECT MAX(version) FROM schema_version",
        [],
        |row| row.get(0),
    )?;
    Ok(version)
}

/// Apply migrations to bring database schema up to date
pub fn apply_migrations(conn: &Connection) -> Result<()> {
    let current_version = get_schema_version(conn)?;

    // Migration to version 2: Add type field to projects
    if current_version < 2 {
        log::info!("Applying migration to version 2: Adding type field to projects");

        // Check if type column already exists
        let has_type_column: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('projects') WHERE name='type'",
                [],
                |row| {
                    let count: i32 = row.get(0)?;
                    Ok(count > 0)
                },
            )?;

        if !has_type_column {
            conn.execute(
                "ALTER TABLE projects ADD COLUMN type TEXT NOT NULL DEFAULT 'Personal'",
                [],
            )?;
        }

        conn.execute(
            "INSERT OR IGNORE INTO schema_version (version, applied_at)
             VALUES (2, datetime('now'))",
            [],
        )?;
    }

    // Migration to version 3: Add updated_at field to notes tables
    if current_version < 3 {
        log::info!("Applying migration to version 3: Adding updated_at field to notes tables");

        // Check and add updated_at to project_notes
        let has_project_notes_updated: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('project_notes') WHERE name='updated_at'",
                [],
                |row| {
                    let count: i32 = row.get(0)?;
                    Ok(count > 0)
                },
            )?;

        if !has_project_notes_updated {
            // Add the column allowing NULL first
            conn.execute(
                "ALTER TABLE project_notes ADD COLUMN updated_at TEXT",
                [],
            )?;
            // Update existing rows to set updated_at = created_at
            conn.execute(
                "UPDATE project_notes SET updated_at = created_at WHERE updated_at IS NULL",
                [],
            )?;
        }

        // Check and add updated_at to milestone_notes
        let has_milestone_notes_updated: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('milestone_notes') WHERE name='updated_at'",
                [],
                |row| {
                    let count: i32 = row.get(0)?;
                    Ok(count > 0)
                },
            )?;

        if !has_milestone_notes_updated {
            // Add the column allowing NULL first
            conn.execute(
                "ALTER TABLE milestone_notes ADD COLUMN updated_at TEXT",
                [],
            )?;
            // Update existing rows to set updated_at = created_at
            conn.execute(
                "UPDATE milestone_notes SET updated_at = created_at WHERE updated_at IS NULL",
                [],
            )?;
        }

        // Check and add updated_at to stakeholder_notes
        let has_stakeholder_notes_updated: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('stakeholder_notes') WHERE name='updated_at'",
                [],
                |row| {
                    let count: i32 = row.get(0)?;
                    Ok(count > 0)
                },
            )?;

        if !has_stakeholder_notes_updated {
            // Add the column allowing NULL first
            conn.execute(
                "ALTER TABLE stakeholder_notes ADD COLUMN updated_at TEXT",
                [],
            )?;
            // Update existing rows to set updated_at = created_at
            conn.execute(
                "UPDATE stakeholder_notes SET updated_at = created_at WHERE updated_at IS NULL",
                [],
            )?;
        }

        conn.execute(
            "INSERT OR IGNORE INTO schema_version (version, applied_at)
             VALUES (3, datetime('now'))",
            [],
        )?;
    }

    log::info!("Database migrations complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_initialize_schema() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        // Verify tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(tables.contains(&"people".to_string()));
        assert!(tables.contains(&"projects".to_string()));
        assert!(tables.contains(&"milestones".to_string()));
        assert!(tables.contains(&"project_stakeholders".to_string()));
        assert!(tables.contains(&"schema_version".to_string()));
    }

    #[test]
    fn test_schema_version() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        let version = get_schema_version(&conn).unwrap();
        assert_eq!(version, 1);
    }
}
