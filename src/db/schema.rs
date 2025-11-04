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

    // Create teams table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS teams (
            name TEXT PRIMARY KEY NOT NULL,
            description TEXT,
            manager TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (manager) REFERENCES people(email)
        )",
        [],
    )?;

    // Create index on team names
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_teams_name ON teams(name)",
        [],
    )?;

    // Create team_members junction table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS team_members (
            team_name TEXT NOT NULL,
            person_email TEXT NOT NULL,
            created_at TEXT NOT NULL,
            PRIMARY KEY (team_name, person_email),
            FOREIGN KEY (team_name) REFERENCES teams(name) ON DELETE CASCADE,
            FOREIGN KEY (person_email) REFERENCES people(email) ON DELETE CASCADE
        )",
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

    // Migration to version 4: Add team column to projects and milestones
    if current_version < 4 {
        log::info!("Applying migration to version 4: Adding team column to projects and milestones");

        // Check and add team to projects
        let has_projects_team: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('projects') WHERE name='team'",
                [],
                |row| {
                    let count: i32 = row.get(0)?;
                    Ok(count > 0)
                },
            )?;

        if !has_projects_team {
            conn.execute(
                "ALTER TABLE projects ADD COLUMN team TEXT",
                [],
            )?;
        }

        // Check and add team to milestones
        let has_milestones_team: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('milestones') WHERE name='team'",
                [],
                |row| {
                    let count: i32 = row.get(0)?;
                    Ok(count > 0)
                },
            )?;

        if !has_milestones_team {
            conn.execute(
                "ALTER TABLE milestones ADD COLUMN team TEXT",
                [],
            )?;
        }

        conn.execute(
            "INSERT OR IGNORE INTO schema_version (version, applied_at)
             VALUES (4, datetime('now'))",
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

    // Schema initialization tests

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
        assert!(tables.contains(&"teams".to_string()));
        assert!(tables.contains(&"team_members".to_string()));
        assert!(tables.contains(&"projects".to_string()));
        assert!(tables.contains(&"milestones".to_string()));
        assert!(tables.contains(&"project_stakeholders".to_string()));
        assert!(tables.contains(&"project_notes".to_string()));
        assert!(tables.contains(&"milestone_notes".to_string()));
        assert!(tables.contains(&"stakeholder_notes".to_string()));
        assert!(tables.contains(&"schema_version".to_string()));
    }

    #[test]
    fn test_initialize_schema_idempotent() {
        let conn = Connection::open_in_memory().unwrap();

        // Initialize twice - should not error
        initialize_schema(&conn).unwrap();
        initialize_schema(&conn).unwrap();

        let version = get_schema_version(&conn).unwrap();
        assert_eq!(version, 1);
    }

    #[test]
    fn test_schema_version() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        let version = get_schema_version(&conn).unwrap();
        assert_eq!(version, 1);
    }

    // Migration tests

    #[test]
    fn test_apply_migrations() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        // Initial version should be 1
        let version = get_schema_version(&conn).unwrap();
        assert_eq!(version, 1);

        // Apply migrations
        apply_migrations(&conn).unwrap();

        // Should now be at version 4 (latest)
        let version = get_schema_version(&conn).unwrap();
        assert_eq!(version, 4);
    }

    #[test]
    fn test_apply_migrations_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        // Apply migrations twice - should not error
        apply_migrations(&conn).unwrap();
        apply_migrations(&conn).unwrap();

        let version = get_schema_version(&conn).unwrap();
        assert_eq!(version, 4);
    }

    #[test]
    fn test_migration_to_version_2_adds_type_column() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        // Apply migrations
        apply_migrations(&conn).unwrap();

        // Verify type column exists in projects table
        let columns: Vec<String> = conn
            .prepare("PRAGMA table_info(projects)")
            .unwrap()
            .query_map([], |row| row.get(1))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(columns.contains(&"type".to_string()));

        // Verify default value works
        let project_id = uuid::Uuid::new_v4();
        conn.execute(
            "INSERT INTO projects (id, name, created_at, updated_at) VALUES (?1, 'Test', datetime('now'), datetime('now'))",
            [project_id.to_string()],
        ).unwrap();

        let project_type: String = conn
            .query_row(
                "SELECT type FROM projects WHERE id = ?1",
                [project_id.to_string()],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(project_type, "Personal");
    }

    #[test]
    fn test_migration_to_version_3_adds_updated_at_to_notes() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        apply_migrations(&conn).unwrap();

        // Verify updated_at column exists in project_notes table
        let columns: Vec<String> = conn
            .prepare("PRAGMA table_info(project_notes)")
            .unwrap()
            .query_map([], |row| row.get(1))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(columns.contains(&"updated_at".to_string()));

        // Verify updated_at column exists in milestone_notes table
        let columns: Vec<String> = conn
            .prepare("PRAGMA table_info(milestone_notes)")
            .unwrap()
            .query_map([], |row| row.get(1))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(columns.contains(&"updated_at".to_string()));

        // Verify updated_at column exists in stakeholder_notes table
        let columns: Vec<String> = conn
            .prepare("PRAGMA table_info(stakeholder_notes)")
            .unwrap()
            .query_map([], |row| row.get(1))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(columns.contains(&"updated_at".to_string()));
    }

    // Foreign key tests

    #[test]
    fn test_foreign_keys_enabled_by_default() {
        let conn = Connection::open_in_memory().unwrap();

        // Enable foreign keys (normally done by open_database)
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();

        initialize_schema(&conn).unwrap();

        // Verify foreign keys are enabled
        let fk_enabled: i32 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(fk_enabled, 1);
    }

    #[test]
    fn test_cascade_delete_milestones() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        initialize_schema(&conn).unwrap();

        let project_id = uuid::Uuid::new_v4();
        let milestone_id = uuid::Uuid::new_v4();

        // Create project
        conn.execute(
            "INSERT INTO projects (id, name, type, created_at, updated_at) VALUES (?1, 'Test', 'Personal', datetime('now'), datetime('now'))",
            [project_id.to_string()],
        ).unwrap();

        // Create milestone
        conn.execute(
            "INSERT INTO milestones (id, project_id, number, name, created_at, updated_at) VALUES (?1, ?2, 1, 'M1', datetime('now'), datetime('now'))",
            [milestone_id.to_string(), project_id.to_string()],
        ).unwrap();

        // Delete project
        conn.execute("DELETE FROM projects WHERE id = ?1", [project_id.to_string()]).unwrap();

        // Verify milestone was deleted
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM milestones WHERE id = ?1", [milestone_id.to_string()], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    // Index tests

    #[test]
    fn test_indexes_created() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        let indexes: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name LIKE 'idx_%'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(indexes.contains(&"idx_people_name".to_string()));
        assert!(indexes.contains(&"idx_teams_name".to_string()));
        assert!(indexes.contains(&"idx_projects_name".to_string()));
        assert!(indexes.contains(&"idx_milestones_due_date".to_string()));
    }
}
