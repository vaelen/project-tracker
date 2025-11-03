// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

use super::models::Person;
use anyhow::{Context, Result};
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

    #[test]
    fn test_create_and_find_person() {
        let conn = Connection::open_in_memory().unwrap();
        db::schema::initialize_schema(&conn).unwrap();

        let repo = PersonRepository::new(&conn);
        let person = Person::new("alice@example.com".to_string(), "Alice Smith".to_string());

        repo.create(&person).unwrap();

        let found = repo.find_by_email("alice@example.com").unwrap().unwrap();
        assert_eq!(found.email, "alice@example.com");
        assert_eq!(found.name, "Alice Smith");
    }

    #[test]
    fn test_search_by_name() {
        let conn = Connection::open_in_memory().unwrap();
        db::schema::initialize_schema(&conn).unwrap();

        let repo = PersonRepository::new(&conn);
        repo.create(&Person::new("alice@example.com".to_string(), "Alice Smith".to_string()))
            .unwrap();
        repo.create(&Person::new("bob@example.com".to_string(), "Bob Jones".to_string()))
            .unwrap();

        let results = repo.search_by_name("Alice").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Alice Smith");
    }
}
