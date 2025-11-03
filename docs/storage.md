# Data Storage Documentation

## Overview

Project Tracker uses SQLite for data storage, providing reliable data integrity, efficient querying, and built-in transaction support. All data is stored in a single SQLite database file located within a configurable data directory.

## Storage Location

### Default Location

```
~/.claude-tracker/data/claude-tracker.db
```

The database file is automatically created on first run if it doesn't exist. The location can be customized via the `data_dir` configuration option.

### Directory Structure

```
~/.claude-tracker/
├── config.toml           # Application configuration
└── data/                 # Data storage directory (configurable)
    └── claude-tracker.db # SQLite database
```

## Database Schema

The application uses SQLite with the following schema:

### People Table

Stores information about all people in the system (employees, stakeholders, managers, etc.).

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| email | TEXT | PRIMARY KEY, NOT NULL | Email address (unique identifier) |
| name | TEXT | NOT NULL | Person's full name |
| team | TEXT | | Team or department name |
| manager | TEXT | FOREIGN KEY (people.email) | Manager's email address |
| notes | TEXT | | Additional notes about the person |
| created_at | TEXT | NOT NULL | ISO8601 creation timestamp |
| updated_at | TEXT | NOT NULL | ISO8601 last update timestamp |

**Indexes:**
- `idx_people_name` on `name` - Enables fast autocomplete searches

**Example Query:**
```sql
SELECT * FROM people WHERE name LIKE '%Smith%' ORDER BY name LIMIT 20;
```

---

### Projects Table

Stores project information including ownership, leadership, and timeline.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PRIMARY KEY, NOT NULL | UUID as string |
| name | TEXT | NOT NULL | Project name |
| description | TEXT | | Project description |
| requirements_owner | TEXT | FOREIGN KEY (people.email) | Requirements owner email |
| technical_lead | TEXT | FOREIGN KEY (people.email) | Technical lead email |
| manager | TEXT | FOREIGN KEY (people.email) | Project manager email |
| due_date | TEXT | | ISO8601 due date |
| jira_initiative | TEXT | | Jira initiative ticket number (e.g., "PROJ-123") |
| created_at | TEXT | NOT NULL | ISO8601 creation timestamp |
| updated_at | TEXT | NOT NULL | ISO8601 last update timestamp |

**Indexes:**
- `idx_projects_name` on `name` - Enables fast project name searches

**Notes:**
- Only the Jira ticket number is stored (not the full URL)
- The application constructs full URLs using the `jira_url` config setting

---

### Milestones Table

Stores project milestones with design docs, technical ownership, and Jira epics.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PRIMARY KEY, NOT NULL | UUID as string |
| project_id | TEXT | FOREIGN KEY (projects.id) ON DELETE CASCADE, NOT NULL | Parent project UUID |
| number | INTEGER | NOT NULL | Milestone number (for ordering) |
| name | TEXT | NOT NULL | Milestone name |
| description | TEXT | | Milestone description |
| technical_lead | TEXT | FOREIGN KEY (people.email) | Tech lead email |
| design_doc_url | TEXT | | Link to design document |
| due_date | TEXT | | ISO8601 due date |
| jira_epic | TEXT | | Jira epic ticket number (e.g., "PROJ-456") |
| created_at | TEXT | NOT NULL | ISO8601 creation timestamp |
| updated_at | TEXT | NOT NULL | ISO8601 last update timestamp |

**Constraints:**
- `UNIQUE(project_id, number)` - Milestone numbers are unique within each project

**Indexes:**
- `idx_milestones_due_date` on `due_date` - Enables fast deadline queries

**Cascading:**
- When a project is deleted, all its milestones are automatically deleted

---

### Project Stakeholders Table

Junction table linking projects to their stakeholders (many-to-many relationship).

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| project_id | TEXT | FOREIGN KEY (projects.id) ON DELETE CASCADE, NOT NULL | Project UUID |
| stakeholder_email | TEXT | FOREIGN KEY (people.email), NOT NULL | Stakeholder email |
| role | TEXT | | Stakeholder's role in the project |
| created_at | TEXT | NOT NULL | ISO8601 creation timestamp |

**Constraints:**
- `PRIMARY KEY (project_id, stakeholder_email)` - One stakeholder can only be added once per project

**Cascading:**
- When a project is deleted, all stakeholder relationships are automatically deleted

---

### Schema Version Table

Tracks database schema version for migrations.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| version | INTEGER | PRIMARY KEY, NOT NULL | Schema version number |
| applied_at | TEXT | NOT NULL | ISO8601 timestamp when version was applied |

**Current Version:** 1

## Data Types and Formats

### Timestamps

All timestamps are stored as TEXT in ISO 8601 format:
```
2025-01-20T14:30:00Z
```

SQLite supports comparison operations on ISO 8601 strings, enabling efficient date-based queries.

### UUIDs

UUIDs are stored as TEXT in the standard hyphenated format:
```
550e8400-e29b-41d4-a716-446655440000
```

### Jira Ticket References

Only ticket numbers are stored (e.g., `PROJ-123`), not full URLs. The application constructs full URLs by combining the stored ticket number with the `jira_url` configuration setting:

```
Stored: "PROJ-123"
Config: jira_url = "https://jira.company.com/browse/"
Result: "https://jira.company.com/browse/PROJ-123"
```

This approach allows changing the Jira instance URL without modifying all data.

## Foreign Key Relationships

The database uses foreign keys to maintain referential integrity:

```
people.manager → people.email
projects.requirements_owner → people.email
projects.technical_lead → people.email
projects.manager → people.email
milestones.project_id → projects.id (CASCADE DELETE)
milestones.technical_lead → people.email
project_stakeholders.project_id → projects.id (CASCADE DELETE)
project_stakeholders.stakeholder_email → people.email
```

**Foreign Key Enforcement:**
- Foreign keys are enabled via `PRAGMA foreign_keys = ON`
- Attempts to reference non-existent people or projects will fail
- Deleting a project automatically deletes its milestones and stakeholder relationships

## Database Initialization

The database is automatically initialized on first connection with:
1. Schema creation (tables, indexes, constraints)
2. Foreign key enforcement enabled
3. Initial schema version recorded

**Initialization Code:**
```rust
use claude_tracker::db;

let conn = db::open_database(&config.database_path()?)?;
```

## Querying Data

### Using Repositories

The application provides repository classes for common operations:

```rust
use claude_tracker::db::{PersonRepository, ProjectRepository};

// Create repositories
let person_repo = PersonRepository::new(&conn);
let project_repo = ProjectRepository::new(&conn);

// Search for people (autocomplete)
let people = person_repo.search_by_name("Alice")?;

// Get project with all details
let project = project_repo.find_by_id(&project_id)?;
let stakeholders = project_repo.get_stakeholders(&project_id)?;
let milestones = project_repo.get_milestones(&project_id)?;
```

### Direct SQL Queries

For advanced queries, you can use SQL directly:

```rust
// Find all projects due this month
let mut stmt = conn.prepare(
    "SELECT * FROM projects
     WHERE due_date BETWEEN ?1 AND ?2
     ORDER BY due_date"
)?;
```

## Data Integrity

### Atomicity

SQLite provides ACID transactions:
- All operations are atomic
- Failed operations are automatically rolled back
- Concurrent access is safely handled

### Constraints

The schema enforces:
- Required fields (NOT NULL)
- Unique constraints (PRIMARY KEY, UNIQUE)
- Foreign key relationships
- Valid email addresses as person identifiers

### Validation

The application performs additional validation:
- Email format validation
- UUID format validation
- ISO 8601 timestamp format
- Jira ticket number format

## Backup and Recovery

### Backup Strategies

**1. File-based Backup**
```bash
# Simple copy (when app is not running)
cp ~/.claude-tracker/data/claude-tracker.db ~/.claude-tracker/backup.db

# Or create dated backups
cp ~/.claude-tracker/data/claude-tracker.db \
   ~/.claude-tracker/backups/backup-$(date +%Y%m%d).db
```

**2. SQLite Backup Command**
```bash
# Online backup (safe while app is running)
sqlite3 ~/.claude-tracker/data/claude-tracker.db \
  ".backup ~/.claude-tracker/backup.db"
```

**3. Export to SQL**
```bash
# Export entire database as SQL
sqlite3 ~/.claude-tracker/data/claude-tracker.db \
  .dump > claude-tracker-export.sql

# Restore from SQL
sqlite3 ~/.claude-tracker/data/claude-tracker-restored.db \
  < claude-tracker-export.sql
```

### Version Control

The SQLite database file is binary and should **not** be committed to version control. Instead:

1. Use the export-to-SQL method for versioning data
2. Add to `.gitignore`:
   ```
   *.db
   *.db-journal
   *.db-wal
   ```

## Performance Considerations

### Indexes

The schema includes indexes on frequently queried columns:
- `people.name` - For autocomplete searches
- `projects.name` - For project searches
- `milestones.due_date` - For deadline queries

### Query Optimization

**Good Practices:**
- Use indexed columns in WHERE clauses
- Limit result sets with LIMIT
- Use foreign key relationships instead of JOINs where possible

**Example:**
```sql
-- Fast: Uses index
SELECT * FROM people WHERE name LIKE 'Smith%' LIMIT 10;

-- Slower: Full table scan
SELECT * FROM people WHERE notes LIKE '%important%';
```

### Database Size

Expected database size for typical usage:
- 100 people: ~50 KB
- 50 projects with 200 milestones: ~100 KB
- Total for small team: < 1 MB

SQLite efficiently handles databases up to several GB.

## Migrations

### Schema Versioning

The `schema_version` table tracks the current schema version. Future updates will include migration scripts:

```sql
-- Check current version
SELECT MAX(version) FROM schema_version;

-- After migration
INSERT INTO schema_version (version, applied_at)
VALUES (2, datetime('now'));
```

### Future Migration Example

```rust
// Pseudo-code for future migrations
fn migrate_to_version_2(conn: &Connection) -> Result<()> {
    conn.execute("ALTER TABLE projects ADD COLUMN status TEXT", [])?;
    conn.execute(
        "INSERT INTO schema_version (version, applied_at)
         VALUES (2, datetime('now'))",
        []
    )?;
    Ok(())
}
```

## Troubleshooting

### Database Locked

**Problem:** "database is locked" error

**Solutions:**
- Close other connections to the database
- Wait for long-running queries to complete
- Check for crashed processes holding locks

```bash
# Check for processes using the database
lsof ~/.claude-tracker/data/claude-tracker.db
```

### Corruption

**Problem:** Database corruption

**Solutions:**
1. Restore from backup
2. Try SQLite's integrity check:
   ```bash
   sqlite3 ~/.claude-tracker/data/claude-tracker.db "PRAGMA integrity_check"
   ```
3. Export and reimport:
   ```bash
   sqlite3 ~/.claude-tracker/data/claude-tracker.db .dump | \
     sqlite3 ~/.claude-tracker/data/repaired.db
   ```

### Foreign Key Violations

**Problem:** Foreign key constraint failed

**Cause:** Trying to reference a non-existent person or project

**Solution:** Ensure referenced entities exist first:
```rust
// Create person first
person_repo.create(&person)?;

// Then create project with person as manager
let mut project = Project::new("Project Name".to_string());
project.manager = Some(person.email.clone());
project_repo.create(&project)?;
```

## Best Practices

1. **Always use transactions** for multiple related operations
2. **Check foreign key constraints** before inserting
3. **Use repositories** instead of raw SQL when possible
4. **Regular backups** - Automate daily backups
5. **Test migrations** on a copy before applying to production data
6. **Monitor database size** and archive old data if needed
7. **Use prepared statements** to prevent SQL injection

## Command-Line Tools

### Inspect Database

```bash
# Open database in SQLite shell
sqlite3 ~/.claude-tracker/data/claude-tracker.db

# Common commands:
.tables                    # List all tables
.schema people            # Show table schema
.mode column              # Better output formatting
.headers on               # Show column headers
SELECT * FROM people;     # Query data
.quit                     # Exit
```

### Database Statistics

```bash
sqlite3 ~/.claude-tracker/data/claude-tracker.db << EOF
SELECT 'People:' as table_name, COUNT(*) as count FROM people
UNION ALL
SELECT 'Projects:', COUNT(*) FROM projects
UNION ALL
SELECT 'Milestones:', COUNT(*) FROM milestones;
EOF
```

## See Also

- [Configuration Documentation](config.md) - Database location and Jira configuration
- [README.md](../README.md) - General application documentation
- [SQLite Documentation](https://www.sqlite.org/docs.html) - Official SQLite documentation
