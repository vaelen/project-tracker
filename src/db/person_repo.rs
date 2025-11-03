// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

use super::models::Person;
use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};

/// Person repository for database operations
pub struct PersonRepository<'a> {
    conn: &'a Connection,
}

impl<'a> PersonRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Create a new person
    pub fn create(&self, person: &Person) -> Result<()> {
        self.conn.execute(
            "INSERT INTO people (email, name, team, manager, notes, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &person.email,
                &person.name,
                &person.team,
                &person.manager,
                &person.notes,
                person.created_at.to_rfc3339(),
                person.updated_at.to_rfc3339(),
            ],
        )?;
        log::debug!("Created person: {}", person.email);
        Ok(())
    }

    /// Find a person by email
    pub fn find_by_email(&self, email: &str) -> Result<Option<Person>> {
        let person = self
            .conn
            .query_row(
                "SELECT email, name, team, manager, notes, created_at, updated_at
                 FROM people WHERE email = ?1",
                params![email],
                |row| {
                    Ok(Person {
                        email: row.get(0)?,
                        name: row.get(1)?,
                        team: row.get(2)?,
                        manager: row.get(3)?,
                        notes: row.get(4)?,
                        created_at: row.get(5)?,
                        updated_at: row.get(6)?,
                    })
                },
            )
            .optional()?;
        Ok(person)
    }

    /// List all people
    pub fn list_all(&self) -> Result<Vec<Person>> {
        let mut stmt = self.conn.prepare(
            "SELECT email, name, team, manager, notes, created_at, updated_at
             FROM people ORDER BY name",
        )?;

        let people = stmt
            .query_map([], |row| {
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

        Ok(people)
    }

    /// Search people by name (for autocomplete)
    pub fn search_by_name(&self, query: &str) -> Result<Vec<Person>> {
        let search_pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT email, name, team, manager, notes, created_at, updated_at
             FROM people WHERE name LIKE ?1 ORDER BY name LIMIT 20",
        )?;

        let people = stmt
            .query_map(params![search_pattern], |row| {
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

        Ok(people)
    }

    /// Update a person
    pub fn update(&self, person: &Person) -> Result<()> {
        let rows = self.conn.execute(
            "UPDATE people SET name = ?1, team = ?2, manager = ?3, notes = ?4, updated_at = ?5
             WHERE email = ?6",
            params![
                &person.name,
                &person.team,
                &person.manager,
                &person.notes,
                Utc::now().to_rfc3339(),
                &person.email,
            ],
        )?;

        if rows == 0 {
            anyhow::bail!("Person not found: {}", person.email);
        }

        log::debug!("Updated person: {}", person.email);
        Ok(())
    }

    /// Delete a person
    pub fn delete(&self, email: &str) -> Result<()> {
        let rows = self.conn.execute("DELETE FROM people WHERE email = ?1", params![email])?;

        if rows == 0 {
            anyhow::bail!("Person not found: {}", email);
        }

        log::debug!("Deleted person: {}", email);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        db::schema::initialize_schema(&conn).unwrap();
        db::schema::apply_migrations(&conn).unwrap();
        conn
    }

    // Person CRUD tests

    #[test]
    fn test_create_and_find_person() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);
        let person = Person::new("alice@example.com".to_string(), "Alice Smith".to_string());

        repo.create(&person).unwrap();

        let found = repo.find_by_email("alice@example.com").unwrap().unwrap();
        assert_eq!(found.email, "alice@example.com");
        assert_eq!(found.name, "Alice Smith");
    }

    #[test]
    fn test_find_nonexistent_person() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        let result = repo.find_by_email("nonexistent@example.com").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_create_person_with_all_fields() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        let mut person = Person::new("alice@example.com".to_string(), "Alice Smith".to_string());
        person.team = Some("Engineering".to_string());
        person.notes = Some("Team lead".to_string());

        repo.create(&person).unwrap();

        let found = repo.find_by_email("alice@example.com").unwrap().unwrap();
        assert_eq!(found.team, Some("Engineering".to_string()));
        assert_eq!(found.notes, Some("Team lead".to_string()));
    }

    #[test]
    fn test_list_all_people() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        repo.create(&Person::new("alice@example.com".to_string(), "Alice Smith".to_string()))
            .unwrap();
        repo.create(&Person::new("bob@example.com".to_string(), "Bob Jones".to_string()))
            .unwrap();
        repo.create(&Person::new("charlie@example.com".to_string(), "Charlie Brown".to_string()))
            .unwrap();

        let people = repo.list_all().unwrap();
        assert_eq!(people.len(), 3);
        // Should be sorted by name
        assert_eq!(people[0].name, "Alice Smith");
        assert_eq!(people[1].name, "Bob Jones");
        assert_eq!(people[2].name, "Charlie Brown");
    }

    #[test]
    fn test_list_all_people_empty() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        let people = repo.list_all().unwrap();
        assert_eq!(people.len(), 0);
    }

    #[test]
    fn test_update_person() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);
        let mut person = Person::new("alice@example.com".to_string(), "Alice Smith".to_string());
        repo.create(&person).unwrap();

        person.name = "Alice Johnson".to_string();
        person.team = Some("Product".to_string());
        person.notes = Some("Promoted to manager".to_string());
        repo.update(&person).unwrap();

        let found = repo.find_by_email("alice@example.com").unwrap().unwrap();
        assert_eq!(found.name, "Alice Johnson");
        assert_eq!(found.team, Some("Product".to_string()));
        assert_eq!(found.notes, Some("Promoted to manager".to_string()));
    }

    #[test]
    fn test_update_nonexistent_person() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);
        let person = Person::new("nonexistent@example.com".to_string(), "Test Person".to_string());

        let result = repo.update(&person);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_delete_person() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);
        let person = Person::new("alice@example.com".to_string(), "Alice Smith".to_string());
        repo.create(&person).unwrap();

        repo.delete("alice@example.com").unwrap();

        let found = repo.find_by_email("alice@example.com").unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_delete_nonexistent_person() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        let result = repo.delete("nonexistent@example.com");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    // Search tests

    #[test]
    fn test_search_by_name() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        repo.create(&Person::new("alice@example.com".to_string(), "Alice Smith".to_string()))
            .unwrap();
        repo.create(&Person::new("bob@example.com".to_string(), "Bob Jones".to_string()))
            .unwrap();
        repo.create(&Person::new("charlie@example.com".to_string(), "Charlie Brown".to_string()))
            .unwrap();

        let results = repo.search_by_name("Alice").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Alice Smith");
    }

    #[test]
    fn test_search_by_name_partial() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        repo.create(&Person::new("alice@example.com".to_string(), "Alice Smith".to_string()))
            .unwrap();
        repo.create(&Person::new("alicia@example.com".to_string(), "Alicia Jones".to_string()))
            .unwrap();
        repo.create(&Person::new("bob@example.com".to_string(), "Bob Alice".to_string()))
            .unwrap();

        let results = repo.search_by_name("Ali").unwrap();
        assert_eq!(results.len(), 3); // Matches Alice, Alicia, and "Bob Alice"
    }

    #[test]
    fn test_search_by_name_case_insensitive() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        repo.create(&Person::new("alice@example.com".to_string(), "Alice Smith".to_string()))
            .unwrap();

        let results = repo.search_by_name("alice").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Alice Smith");
    }

    #[test]
    fn test_search_by_name_no_results() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        repo.create(&Person::new("alice@example.com".to_string(), "Alice Smith".to_string()))
            .unwrap();

        let results = repo.search_by_name("Bob").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_by_name_limit() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        // Create 25 people with similar names
        for i in 0..25 {
            let person = Person::new(
                format!("person{}@example.com", i),
                format!("Test Person {}", i)
            );
            repo.create(&person).unwrap();
        }

        let results = repo.search_by_name("Test Person").unwrap();
        // Should limit to 20 results
        assert_eq!(results.len(), 20);
    }

    // Manager relationship tests

    #[test]
    fn test_create_person_with_manager() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        // Create manager first
        let manager = Person::new("manager@example.com".to_string(), "Manager".to_string());
        repo.create(&manager).unwrap();

        // Create employee with manager
        let mut employee = Person::new("employee@example.com".to_string(), "Employee".to_string());
        employee.manager = Some("manager@example.com".to_string());
        repo.create(&employee).unwrap();

        let found = repo.find_by_email("employee@example.com").unwrap().unwrap();
        assert_eq!(found.manager, Some("manager@example.com".to_string()));
    }

    #[test]
    fn test_create_duplicate_email_fails() {
        let conn = setup_test_db();
        let repo = PersonRepository::new(&conn);

        let person1 = Person::new("alice@example.com".to_string(), "Alice Smith".to_string());
        repo.create(&person1).unwrap();

        let person2 = Person::new("alice@example.com".to_string(), "Alice Jones".to_string());
        let result = repo.create(&person2);
        assert!(result.is_err());
    }
}
