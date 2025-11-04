// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

use super::models::{Team, Person};
use anyhow::{anyhow, Result};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};

/// Team repository for database operations
pub struct TeamRepository<'a> {
    conn: &'a Connection,
}

impl<'a> TeamRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Create a new team
    pub fn create(&self, team: &Team) -> Result<()> {
        self.conn.execute(
            "INSERT INTO teams (name, description, manager, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &team.name,
                &team.description,
                &team.manager,
                team.created_at.to_rfc3339(),
                team.updated_at.to_rfc3339(),
            ],
        )?;
        log::debug!("Created team: {}", team.name);
        Ok(())
    }

    /// Find a team by name
    pub fn find_by_name(&self, name: &str) -> Result<Option<Team>> {
        let team = self
            .conn
            .query_row(
                "SELECT name, description, manager, created_at, updated_at
                 FROM teams WHERE name = ?1",
                params![name],
                |row| {
                    Ok(Team {
                        name: row.get(0)?,
                        description: row.get(1)?,
                        manager: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                },
            )
            .optional()?;
        Ok(team)
    }

    /// List all teams
    pub fn list_all(&self) -> Result<Vec<Team>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, description, manager, created_at, updated_at
             FROM teams ORDER BY name",
        )?;

        let teams = stmt
            .query_map([], |row| {
                Ok(Team {
                    name: row.get(0)?,
                    description: row.get(1)?,
                    manager: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(teams)
    }

    /// Search teams by name (for autocomplete)
    pub fn search_by_name(&self, query: &str) -> Result<Vec<Team>> {
        let search_pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT name, description, manager, created_at, updated_at
             FROM teams WHERE name LIKE ?1 ORDER BY name LIMIT 20",
        )?;

        let teams = stmt
            .query_map(params![search_pattern], |row| {
                Ok(Team {
                    name: row.get(0)?,
                    description: row.get(1)?,
                    manager: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(teams)
    }

    /// Update a team
    pub fn update(&self, team: &Team) -> Result<()> {
        let rows = self.conn.execute(
            "UPDATE teams SET description = ?1, manager = ?2, updated_at = ?3
             WHERE name = ?4",
            params![
                &team.description,
                &team.manager,
                Utc::now().to_rfc3339(),
                &team.name,
            ],
        )?;

        if rows == 0 {
            return Err(anyhow!("Team not found: {}", team.name));
        }

        log::debug!("Updated team: {}", team.name);
        Ok(())
    }

    /// Delete a team
    pub fn delete(&self, name: &str) -> Result<()> {
        let rows = self.conn.execute("DELETE FROM teams WHERE name = ?1", params![name])?;

        if rows == 0 {
            return Err(anyhow!("Team not found: {}", name));
        }

        log::debug!("Deleted team: {}", name);
        Ok(())
    }

    /// Add a member to a team
    pub fn add_member(&self, team_name: &str, person_email: &str) -> Result<()> {
        // Verify team exists
        if self.find_by_name(team_name)?.is_none() {
            return Err(anyhow!("Team not found: {}", team_name));
        }

        // Verify person exists
        let person_exists: bool = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM people WHERE email = ?1",
                params![person_email],
                |row| {
                    let count: i32 = row.get(0)?;
                    Ok(count > 0)
                },
            )?;

        if !person_exists {
            return Err(anyhow!("Person not found: {}", person_email));
        }

        self.conn.execute(
            "INSERT INTO team_members (team_name, person_email, created_at)
             VALUES (?1, ?2, ?3)",
            params![
                team_name,
                person_email,
                Utc::now().to_rfc3339(),
            ],
        )?;

        log::debug!("Added {} to team {}", person_email, team_name);
        Ok(())
    }

    /// Remove a member from a team
    pub fn remove_member(&self, team_name: &str, person_email: &str) -> Result<()> {
        let rows = self.conn.execute(
            "DELETE FROM team_members WHERE team_name = ?1 AND person_email = ?2",
            params![team_name, person_email],
        )?;

        if rows == 0 {
            return Err(anyhow!(
                "Team member not found: {} in team {}",
                person_email,
                team_name
            ));
        }

        log::debug!("Removed {} from team {}", person_email, team_name);
        Ok(())
    }

    /// Get all members of a team
    pub fn get_members(&self, team_name: &str) -> Result<Vec<Person>> {
        let mut stmt = self.conn.prepare(
            "SELECT p.email, p.name, p.team, p.manager, p.notes, p.created_at, p.updated_at
             FROM people p
             INNER JOIN team_members tm ON p.email = tm.person_email
             WHERE tm.team_name = ?1
             ORDER BY p.name",
        )?;

        let members = stmt
            .query_map(params![team_name], |row| {
                Ok(Person {
                    email: row.get(0)?,
                    name: row.get(1)?,
                    team: row.get(2)?,
                    manager: row.get(3)?,
                    notes: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(members)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{open_database, Person};
    use tempfile::tempdir;

    fn setup_test_db() -> (tempfile::TempDir, Connection) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = open_database(&db_path).unwrap();
        (dir, conn)
    }

    #[test]
    fn test_create_team() {
        let (_dir, conn) = setup_test_db();
        let repo = TeamRepository::new(&conn);

        let team = Team::new("Engineering".to_string());
        repo.create(&team).unwrap();

        let found = repo.find_by_name("Engineering").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Engineering");
    }

    #[test]
    fn test_create_duplicate_team() {
        let (_dir, conn) = setup_test_db();
        let repo = TeamRepository::new(&conn);

        let team = Team::new("Engineering".to_string());
        repo.create(&team).unwrap();

        // Try to create duplicate
        let result = repo.create(&team);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_team_not_found() {
        let (_dir, conn) = setup_test_db();
        let repo = TeamRepository::new(&conn);

        let found = repo.find_by_name("Nonexistent").unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_list_all_teams() {
        let (_dir, conn) = setup_test_db();
        let repo = TeamRepository::new(&conn);

        // Initially empty
        let teams = repo.list_all().unwrap();
        assert_eq!(teams.len(), 0);

        // Add teams
        repo.create(&Team::new("Engineering".to_string())).unwrap();
        repo.create(&Team::new("Product".to_string())).unwrap();
        repo.create(&Team::new("Design".to_string())).unwrap();

        // Should be sorted by name
        let teams = repo.list_all().unwrap();
        assert_eq!(teams.len(), 3);
        assert_eq!(teams[0].name, "Design");
        assert_eq!(teams[1].name, "Engineering");
        assert_eq!(teams[2].name, "Product");
    }

    #[test]
    fn test_search_teams() {
        let (_dir, conn) = setup_test_db();
        let repo = TeamRepository::new(&conn);

        repo.create(&Team::new("Engineering".to_string())).unwrap();
        repo.create(&Team::new("Product".to_string())).unwrap();
        repo.create(&Team::new("Marketing".to_string())).unwrap();

        // Search for teams with "ing"
        let teams = repo.search_by_name("ing").unwrap();
        assert_eq!(teams.len(), 2); // Engineering, Marketing

        // Search for teams with "prod"
        let teams = repo.search_by_name("prod").unwrap();
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].name, "Product");
    }

    #[test]
    fn test_update_team() {
        let (_dir, conn) = setup_test_db();
        let repo = TeamRepository::new(&conn);

        let mut team = Team::new("Engineering".to_string());
        team.description = Some("Software engineering team".to_string());
        repo.create(&team).unwrap();

        // Update description
        team.description = Some("Updated description".to_string());
        repo.update(&team).unwrap();

        let found = repo.find_by_name("Engineering").unwrap().unwrap();
        assert_eq!(found.description, Some("Updated description".to_string()));
    }

    #[test]
    fn test_update_nonexistent_team() {
        let (_dir, conn) = setup_test_db();
        let repo = TeamRepository::new(&conn);

        let team = Team::new("Nonexistent".to_string());
        let result = repo.update(&team);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_team() {
        let (_dir, conn) = setup_test_db();
        let repo = TeamRepository::new(&conn);

        let team = Team::new("Engineering".to_string());
        repo.create(&team).unwrap();

        repo.delete("Engineering").unwrap();

        let found = repo.find_by_name("Engineering").unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_delete_nonexistent_team() {
        let (_dir, conn) = setup_test_db();
        let repo = TeamRepository::new(&conn);

        let result = repo.delete("Nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_team_member() {
        let (_dir, conn) = setup_test_db();
        let team_repo = TeamRepository::new(&conn);
        let person_repo = crate::db::PersonRepository::new(&conn);

        // Create team and person
        let team = Team::new("Engineering".to_string());
        team_repo.create(&team).unwrap();

        let person = Person::new("test@example.com".to_string(), "Test User".to_string());
        person_repo.create(&person).unwrap();

        // Add member
        team_repo.add_member("Engineering", "test@example.com").unwrap();

        // Verify member was added
        let members = team_repo.get_members("Engineering").unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].email, "test@example.com");
    }

    #[test]
    fn test_add_member_to_nonexistent_team() {
        let (_dir, conn) = setup_test_db();
        let team_repo = TeamRepository::new(&conn);
        let person_repo = crate::db::PersonRepository::new(&conn);

        let person = Person::new("test@example.com".to_string(), "Test User".to_string());
        person_repo.create(&person).unwrap();

        let result = team_repo.add_member("Nonexistent", "test@example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_nonexistent_person_to_team() {
        let (_dir, conn) = setup_test_db();
        let team_repo = TeamRepository::new(&conn);

        let team = Team::new("Engineering".to_string());
        team_repo.create(&team).unwrap();

        let result = team_repo.add_member("Engineering", "nonexistent@example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_team_member() {
        let (_dir, conn) = setup_test_db();
        let team_repo = TeamRepository::new(&conn);
        let person_repo = crate::db::PersonRepository::new(&conn);

        // Create team and person
        let team = Team::new("Engineering".to_string());
        team_repo.create(&team).unwrap();

        let person = Person::new("test@example.com".to_string(), "Test User".to_string());
        person_repo.create(&person).unwrap();

        // Add and remove member
        team_repo.add_member("Engineering", "test@example.com").unwrap();
        team_repo.remove_member("Engineering", "test@example.com").unwrap();

        // Verify member was removed
        let members = team_repo.get_members("Engineering").unwrap();
        assert_eq!(members.len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_member() {
        let (_dir, conn) = setup_test_db();
        let team_repo = TeamRepository::new(&conn);

        let team = Team::new("Engineering".to_string());
        team_repo.create(&team).unwrap();

        let result = team_repo.remove_member("Engineering", "nonexistent@example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_team_members() {
        let (_dir, conn) = setup_test_db();
        let team_repo = TeamRepository::new(&conn);
        let person_repo = crate::db::PersonRepository::new(&conn);

        // Create team
        let team = Team::new("Engineering".to_string());
        team_repo.create(&team).unwrap();

        // Create multiple people
        person_repo.create(&Person::new("alice@example.com".to_string(), "Alice".to_string())).unwrap();
        person_repo.create(&Person::new("bob@example.com".to_string(), "Bob".to_string())).unwrap();
        person_repo.create(&Person::new("charlie@example.com".to_string(), "Charlie".to_string())).unwrap();

        // Add members
        team_repo.add_member("Engineering", "alice@example.com").unwrap();
        team_repo.add_member("Engineering", "charlie@example.com").unwrap();

        // Get members (should be sorted by name)
        let members = team_repo.get_members("Engineering").unwrap();
        assert_eq!(members.len(), 2);
        assert_eq!(members[0].name, "Alice");
        assert_eq!(members[1].name, "Charlie");
    }

    #[test]
    fn test_delete_team_cascades_to_members() {
        let (_dir, conn) = setup_test_db();
        let team_repo = TeamRepository::new(&conn);
        let person_repo = crate::db::PersonRepository::new(&conn);

        // Create team and person
        let team = Team::new("Engineering".to_string());
        team_repo.create(&team).unwrap();

        let person = Person::new("test@example.com".to_string(), "Test User".to_string());
        person_repo.create(&person).unwrap();

        team_repo.add_member("Engineering", "test@example.com").unwrap();

        // Delete team
        team_repo.delete("Engineering").unwrap();

        // Verify team_members were deleted (check by querying directly)
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM team_members WHERE team_name = 'Engineering'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);

        // Person should still exist
        let person_found = person_repo.find_by_email("test@example.com").unwrap();
        assert!(person_found.is_some());
    }
}
