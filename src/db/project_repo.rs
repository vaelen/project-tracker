// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

use super::models::{Milestone, MilestoneNote, Project, ProjectNote, ProjectStakeholder, StakeholderNote};
use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use uuid::Uuid;

/// Project repository for database operations
pub struct ProjectRepository<'a> {
    conn: &'a Connection,
}

impl<'a> ProjectRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Create a new project
    pub fn create(&self, project: &Project) -> Result<()> {
        self.conn.execute(
            "INSERT INTO projects (id, name, description, type, requirements_owner, technical_lead,
                                  manager, team, due_date, jira_initiative, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                project.id.to_string(),
                &project.name,
                &project.description,
                &project.project_type,
                &project.requirements_owner,
                &project.technical_lead,
                &project.manager,
                &project.team,
                project.due_date.map(|d| d.to_rfc3339()),
                &project.jira_initiative,
                project.created_at.to_rfc3339(),
                project.updated_at.to_rfc3339(),
            ],
        )?;
        log::debug!("Created project: {} ({})", project.name, project.id);
        Ok(())
    }

    /// Find a project by ID
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<Project>> {
        let project = self
            .conn
            .query_row(
                "SELECT id, name, description, type, requirements_owner, technical_lead, manager, team,
                        due_date, jira_initiative, created_at, updated_at
                 FROM projects WHERE id = ?1",
                params![id.to_string()],
                |row| {
                    Ok(Project {
                        id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                        name: row.get(1)?,
                        description: row.get(2)?,
                        project_type: row.get(3)?,
                        requirements_owner: row.get(4)?,
                        technical_lead: row.get(5)?,
                        manager: row.get(6)?,
                        team: row.get(7)?,
                        due_date: row.get(8)?,
                        jira_initiative: row.get(9)?,
                        created_at: row.get(10)?,
                        updated_at: row.get(11)?,
                    })
                },
            )
            .optional()?;
        Ok(project)
    }

    /// List all projects
    pub fn list_all(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, type, requirements_owner, technical_lead, manager, team,
                    due_date, jira_initiative, created_at, updated_at
             FROM projects ORDER BY name",
        )?;

        let projects = stmt
            .query_map([], |row| {
                Ok(Project {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    name: row.get(1)?,
                    description: row.get(2)?,
                    project_type: row.get(3)?,
                    requirements_owner: row.get(4)?,
                    technical_lead: row.get(5)?,
                    manager: row.get(6)?,
                    team: row.get(7)?,
                    due_date: row.get(8)?,
                    jira_initiative: row.get(9)?,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    /// Update a project
    pub fn update(&self, project: &Project) -> Result<()> {
        let rows = self.conn.execute(
            "UPDATE projects SET name = ?1, description = ?2, type = ?3, requirements_owner = ?4,
                                technical_lead = ?5, manager = ?6, team = ?7, due_date = ?8,
                                jira_initiative = ?9, updated_at = ?10
             WHERE id = ?11",
            params![
                &project.name,
                &project.description,
                &project.project_type,
                &project.requirements_owner,
                &project.technical_lead,
                &project.manager,
                &project.team,
                project.due_date.map(|d| d.to_rfc3339()),
                &project.jira_initiative,
                Utc::now().to_rfc3339(),
                project.id.to_string(),
            ],
        )?;

        if rows == 0 {
            anyhow::bail!("Project not found: {}", project.id);
        }

        log::debug!("Updated project: {}", project.id);
        Ok(())
    }

    /// Add stakeholder to project
    pub fn add_stakeholder(&self, project_id: &Uuid, stakeholder: &ProjectStakeholder) -> Result<()> {
        self.conn.execute(
            "INSERT INTO project_stakeholders (project_id, stakeholder_email, role, created_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                project_id.to_string(),
                &stakeholder.stakeholder_email,
                &stakeholder.role,
                stakeholder.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// Get project stakeholders
    pub fn get_stakeholders(&self, project_id: &Uuid) -> Result<Vec<ProjectStakeholder>> {
        let mut stmt = self.conn.prepare(
            "SELECT project_id, stakeholder_email, role, created_at
             FROM project_stakeholders WHERE project_id = ?1",
        )?;

        let stakeholders = stmt
            .query_map(params![project_id.to_string()], |row| {
                Ok(ProjectStakeholder {
                    project_id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    stakeholder_email: row.get(1)?,
                    role: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(stakeholders)
    }

    /// Get project milestones
    pub fn get_milestones(&self, project_id: &Uuid) -> Result<Vec<Milestone>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, number, name, description, technical_lead, team,
                    design_doc_url, due_date, jira_epic, created_at, updated_at
             FROM milestones WHERE project_id = ?1 ORDER BY number",
        )?;

        let milestones = stmt
            .query_map(params![project_id.to_string()], |row| {
                Ok(Milestone {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    project_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    number: row.get(2)?,
                    name: row.get(3)?,
                    description: row.get(4)?,
                    technical_lead: row.get(5)?,
                    team: row.get(6)?,
                    design_doc_url: row.get(7)?,
                    due_date: row.get(8)?,
                    jira_epic: row.get(9)?,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(milestones)
    }

    /// Add milestone to project
    pub fn add_milestone(&self, milestone: &Milestone) -> Result<()> {
        self.conn.execute(
            "INSERT INTO milestones (id, project_id, number, name, description, technical_lead, team,
                                    design_doc_url, due_date, jira_epic, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                milestone.id.to_string(),
                milestone.project_id.to_string(),
                milestone.number,
                &milestone.name,
                &milestone.description,
                &milestone.technical_lead,
                &milestone.team,
                &milestone.design_doc_url,
                milestone.due_date.map(|d| d.to_rfc3339()),
                &milestone.jira_epic,
                milestone.created_at.to_rfc3339(),
                milestone.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// Update a milestone
    pub fn update_milestone(&self, milestone: &Milestone) -> Result<()> {
        let rows = self.conn.execute(
            "UPDATE milestones SET number = ?1, name = ?2, description = ?3, technical_lead = ?4,
                                   team = ?5, design_doc_url = ?6, due_date = ?7, jira_epic = ?8, updated_at = ?9
             WHERE id = ?10",
            params![
                milestone.number,
                &milestone.name,
                &milestone.description,
                &milestone.technical_lead,
                &milestone.team,
                &milestone.design_doc_url,
                milestone.due_date.map(|d| d.to_rfc3339()),
                &milestone.jira_epic,
                Utc::now().to_rfc3339(),
                milestone.id.to_string(),
            ],
        )?;

        if rows == 0 {
            anyhow::bail!("Milestone not found: {}", milestone.id);
        }

        log::debug!("Updated milestone: {}", milestone.id);
        Ok(())
    }

    /// Delete a milestone
    pub fn delete_milestone(&self, id: &Uuid) -> Result<()> {
        let rows = self.conn.execute("DELETE FROM milestones WHERE id = ?1", params![id.to_string()])?;

        if rows == 0 {
            anyhow::bail!("Milestone not found: {}", id);
        }

        log::debug!("Deleted milestone: {}", id);
        Ok(())
    }

    /// Delete a project (cascades to milestones and stakeholders)
    pub fn delete(&self, id: &Uuid) -> Result<()> {
        let rows = self.conn.execute("DELETE FROM projects WHERE id = ?1", params![id.to_string()])?;

        if rows == 0 {
            anyhow::bail!("Project not found: {}", id);
        }

        log::debug!("Deleted project: {}", id);
        Ok(())
    }

    /// Update a stakeholder
    pub fn update_stakeholder(&self, project_id: &Uuid, stakeholder: &ProjectStakeholder) -> Result<()> {
        let rows = self.conn.execute(
            "UPDATE project_stakeholders SET role = ?1 WHERE project_id = ?2 AND stakeholder_email = ?3",
            params![
                &stakeholder.role,
                project_id.to_string(),
                &stakeholder.stakeholder_email,
            ],
        )?;

        if rows == 0 {
            anyhow::bail!("Stakeholder not found");
        }

        Ok(())
    }

    /// Remove stakeholder from project
    pub fn remove_stakeholder(&self, project_id: &Uuid, stakeholder_email: &str) -> Result<()> {
        let rows = self.conn.execute(
            "DELETE FROM project_stakeholders WHERE project_id = ?1 AND stakeholder_email = ?2",
            params![project_id.to_string(), stakeholder_email],
        )?;

        if rows == 0 {
            anyhow::bail!("Stakeholder not found");
        }

        Ok(())
    }

    // Project Notes

    /// Get notes for a project
    pub fn get_project_notes(&self, project_id: &Uuid) -> Result<Vec<ProjectNote>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, title, body, created_at, updated_at
             FROM project_notes WHERE project_id = ?1 ORDER BY created_at DESC",
        )?;

        let notes = stmt
            .query_map(params![project_id.to_string()], |row| {
                Ok(ProjectNote {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    project_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    title: row.get(2)?,
                    body: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(notes)
    }

    /// Add note to project
    pub fn add_project_note(&self, note: &ProjectNote) -> Result<()> {
        self.conn.execute(
            "INSERT INTO project_notes (id, project_id, title, body, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                note.id.to_string(),
                note.project_id.to_string(),
                &note.title,
                &note.body,
                note.created_at.to_rfc3339(),
                note.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// Update a project note
    pub fn update_project_note(&self, note: &ProjectNote) -> Result<()> {
        let rows = self.conn.execute(
            "UPDATE project_notes SET title = ?1, body = ?2, updated_at = ?3
             WHERE id = ?4",
            params![
                &note.title,
                &note.body,
                note.updated_at.to_rfc3339(),
                note.id.to_string(),
            ],
        )?;

        if rows == 0 {
            anyhow::bail!("Project note not found: {}", note.id);
        }

        log::debug!("Updated project note: {}", note.id);
        Ok(())
    }

    /// Delete project note
    pub fn delete_project_note(&self, id: &Uuid) -> Result<()> {
        let rows = self.conn.execute("DELETE FROM project_notes WHERE id = ?1", params![id.to_string()])?;

        if rows == 0 {
            anyhow::bail!("Note not found: {}", id);
        }

        Ok(())
    }

    // Milestone Notes

    /// Get notes for a milestone
    pub fn get_milestone_notes(&self, milestone_id: &Uuid) -> Result<Vec<MilestoneNote>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, milestone_id, title, body, created_at, updated_at
             FROM milestone_notes WHERE milestone_id = ?1 ORDER BY created_at DESC",
        )?;

        let notes = stmt
            .query_map(params![milestone_id.to_string()], |row| {
                Ok(MilestoneNote {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    milestone_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    title: row.get(2)?,
                    body: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(notes)
    }

    /// Add note to milestone
    pub fn add_milestone_note(&self, note: &MilestoneNote) -> Result<()> {
        self.conn.execute(
            "INSERT INTO milestone_notes (id, milestone_id, title, body, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                note.id.to_string(),
                note.milestone_id.to_string(),
                &note.title,
                &note.body,
                note.created_at.to_rfc3339(),
                note.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// Update a milestone note
    pub fn update_milestone_note(&self, note: &MilestoneNote) -> Result<()> {
        let rows = self.conn.execute(
            "UPDATE milestone_notes SET title = ?1, body = ?2, updated_at = ?3
             WHERE id = ?4",
            params![
                &note.title,
                &note.body,
                note.updated_at.to_rfc3339(),
                note.id.to_string(),
            ],
        )?;

        if rows == 0 {
            anyhow::bail!("Milestone note not found: {}", note.id);
        }

        log::debug!("Updated milestone note: {}", note.id);
        Ok(())
    }

    /// Delete milestone note
    pub fn delete_milestone_note(&self, id: &Uuid) -> Result<()> {
        let rows = self.conn.execute("DELETE FROM milestone_notes WHERE id = ?1", params![id.to_string()])?;

        if rows == 0 {
            anyhow::bail!("Note not found: {}", id);
        }

        Ok(())
    }

    // Stakeholder Notes

    /// Get notes for a stakeholder
    pub fn get_stakeholder_notes(&self, project_id: &Uuid, stakeholder_email: &str) -> Result<Vec<StakeholderNote>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, stakeholder_email, title, body, created_at, updated_at
             FROM stakeholder_notes WHERE project_id = ?1 AND stakeholder_email = ?2 ORDER BY created_at DESC",
        )?;

        let notes = stmt
            .query_map(params![project_id.to_string(), stakeholder_email], |row| {
                Ok(StakeholderNote {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    project_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    stakeholder_email: row.get(2)?,
                    title: row.get(3)?,
                    body: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(notes)
    }

    /// Add note to stakeholder
    pub fn add_stakeholder_note(&self, note: &StakeholderNote) -> Result<()> {
        self.conn.execute(
            "INSERT INTO stakeholder_notes (id, project_id, stakeholder_email, title, body, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                note.id.to_string(),
                note.project_id.to_string(),
                &note.stakeholder_email,
                &note.title,
                &note.body,
                note.created_at.to_rfc3339(),
                note.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// Update a stakeholder note
    pub fn update_stakeholder_note(&self, note: &StakeholderNote) -> Result<()> {
        let rows = self.conn.execute(
            "UPDATE stakeholder_notes SET title = ?1, body = ?2, updated_at = ?3
             WHERE id = ?4",
            params![
                &note.title,
                &note.body,
                note.updated_at.to_rfc3339(),
                note.id.to_string(),
            ],
        )?;

        if rows == 0 {
            anyhow::bail!("Stakeholder note not found: {}", note.id);
        }

        log::debug!("Updated stakeholder note: {}", note.id);
        Ok(())
    }

    /// Delete stakeholder note
    pub fn delete_stakeholder_note(&self, id: &Uuid) -> Result<()> {
        let rows = self.conn.execute("DELETE FROM stakeholder_notes WHERE id = ?1", params![id.to_string()])?;

        if rows == 0 {
            anyhow::bail!("Note not found: {}", id);
        }

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

    // Project CRUD tests

    #[test]
    fn test_create_and_find_project() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());

        repo.create(&project).unwrap();

        let found = repo.find_by_id(&project.id).unwrap().unwrap();
        assert_eq!(found.name, "Test Project");
        assert_eq!(found.id, project.id);
    }

    #[test]
    fn test_find_nonexistent_project() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let fake_id = Uuid::new_v4();

        let result = repo.find_by_id(&fake_id).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_list_all_projects() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);

        let project1 = Project::new("Alpha Project".to_string());
        let project2 = Project::new("Beta Project".to_string());
        repo.create(&project1).unwrap();
        repo.create(&project2).unwrap();

        let projects = repo.list_all().unwrap();
        assert_eq!(projects.len(), 2);
        // Should be sorted by name
        assert_eq!(projects[0].name, "Alpha Project");
        assert_eq!(projects[1].name, "Beta Project");
    }

    #[test]
    fn test_update_project() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let mut project = Project::new("Original Name".to_string());
        repo.create(&project).unwrap();

        project.name = "Updated Name".to_string();
        project.description = Some("New description".to_string());
        repo.update(&project).unwrap();

        let found = repo.find_by_id(&project.id).unwrap().unwrap();
        assert_eq!(found.name, "Updated Name");
        assert_eq!(found.description, Some("New description".to_string()));
    }

    #[test]
    fn test_update_nonexistent_project() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());

        let result = repo.update(&project);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_delete_project() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        repo.delete(&project.id).unwrap();

        let found = repo.find_by_id(&project.id).unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_delete_nonexistent_project() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let fake_id = Uuid::new_v4();

        let result = repo.delete(&fake_id);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    // Milestone tests

    #[test]
    fn test_add_milestone() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let milestone = Milestone::new(project.id, 1, "Milestone 1".to_string());
        repo.add_milestone(&milestone).unwrap();

        let milestones = repo.get_milestones(&project.id).unwrap();
        assert_eq!(milestones.len(), 1);
        assert_eq!(milestones[0].name, "Milestone 1");
        assert_eq!(milestones[0].number, 1);
    }

    #[test]
    fn test_get_milestones_empty() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let milestones = repo.get_milestones(&project.id).unwrap();
        assert_eq!(milestones.len(), 0);
    }

    #[test]
    fn test_update_milestone() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let mut milestone = Milestone::new(project.id, 1, "Original Name".to_string());
        repo.add_milestone(&milestone).unwrap();

        milestone.name = "Updated Name".to_string();
        milestone.description = Some("New description".to_string());
        repo.update_milestone(&milestone).unwrap();

        let milestones = repo.get_milestones(&project.id).unwrap();
        assert_eq!(milestones[0].name, "Updated Name");
        assert_eq!(milestones[0].description, Some("New description".to_string()));
    }

    #[test]
    fn test_update_nonexistent_milestone() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let milestone = Milestone::new(project.id, 1, "Test Milestone".to_string());
        let result = repo.update_milestone(&milestone);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_delete_milestone() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let milestone = Milestone::new(project.id, 1, "Milestone 1".to_string());
        repo.add_milestone(&milestone).unwrap();

        repo.delete_milestone(&milestone.id).unwrap();

        let milestones = repo.get_milestones(&project.id).unwrap();
        assert_eq!(milestones.len(), 0);
    }

    #[test]
    fn test_delete_nonexistent_milestone() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let fake_id = Uuid::new_v4();

        let result = repo.delete_milestone(&fake_id);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_delete_project_cascades_to_milestones() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let milestone = Milestone::new(project.id, 1, "Milestone 1".to_string());
        repo.add_milestone(&milestone).unwrap();

        repo.delete(&project.id).unwrap();

        // Milestones should be deleted via cascade
        let milestones = repo.get_milestones(&project.id).unwrap();
        assert_eq!(milestones.len(), 0);
    }

    // Stakeholder tests

    #[test]
    fn test_add_stakeholder() {
        let conn = setup_test_db();
        let person_repo = crate::db::PersonRepository::new(&conn);
        let repo = ProjectRepository::new(&conn);

        let person = crate::db::Person::new("alice@example.com".to_string(), "Alice".to_string());
        person_repo.create(&person).unwrap();

        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let mut stakeholder = ProjectStakeholder::new(project.id, "alice@example.com".to_string());
        stakeholder.role = Some("Lead".to_string());
        repo.add_stakeholder(&project.id, &stakeholder).unwrap();

        let stakeholders = repo.get_stakeholders(&project.id).unwrap();
        assert_eq!(stakeholders.len(), 1);
        assert_eq!(stakeholders[0].stakeholder_email, "alice@example.com");
        assert_eq!(stakeholders[0].role, Some("Lead".to_string()));
    }

    #[test]
    fn test_update_stakeholder() {
        let conn = setup_test_db();
        let person_repo = crate::db::PersonRepository::new(&conn);
        let repo = ProjectRepository::new(&conn);

        let person = crate::db::Person::new("alice@example.com".to_string(), "Alice".to_string());
        person_repo.create(&person).unwrap();

        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let mut stakeholder = ProjectStakeholder::new(project.id, "alice@example.com".to_string());
        stakeholder.role = Some("Member".to_string());
        repo.add_stakeholder(&project.id, &stakeholder).unwrap();

        stakeholder.role = Some("Lead".to_string());
        repo.update_stakeholder(&project.id, &stakeholder).unwrap();

        let stakeholders = repo.get_stakeholders(&project.id).unwrap();
        assert_eq!(stakeholders[0].role, Some("Lead".to_string()));
    }

    #[test]
    fn test_remove_stakeholder() {
        let conn = setup_test_db();
        let person_repo = crate::db::PersonRepository::new(&conn);
        let repo = ProjectRepository::new(&conn);

        let person = crate::db::Person::new("alice@example.com".to_string(), "Alice".to_string());
        person_repo.create(&person).unwrap();

        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let stakeholder = ProjectStakeholder::new(project.id, "alice@example.com".to_string());
        repo.add_stakeholder(&project.id, &stakeholder).unwrap();

        repo.remove_stakeholder(&project.id, "alice@example.com").unwrap();

        let stakeholders = repo.get_stakeholders(&project.id).unwrap();
        assert_eq!(stakeholders.len(), 0);
    }

    // Project Notes tests

    #[test]
    fn test_add_project_note() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let note = ProjectNote::new(project.id, "Test Note".to_string(), "Note body".to_string());
        repo.add_project_note(&note).unwrap();

        let notes = repo.get_project_notes(&project.id).unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].title, "Test Note");
        assert_eq!(notes[0].body, "Note body");
    }

    #[test]
    fn test_update_project_note() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let mut note = ProjectNote::new(project.id, "Original Title".to_string(), "Original body".to_string());
        repo.add_project_note(&note).unwrap();

        note.title = "Updated Title".to_string();
        note.body = "Updated body".to_string();
        repo.update_project_note(&note).unwrap();

        let notes = repo.get_project_notes(&project.id).unwrap();
        assert_eq!(notes[0].title, "Updated Title");
        assert_eq!(notes[0].body, "Updated body");
    }

    #[test]
    fn test_delete_project_note() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let note = ProjectNote::new(project.id, "Test Note".to_string(), "Note body".to_string());
        repo.add_project_note(&note).unwrap();

        repo.delete_project_note(&note.id).unwrap();

        let notes = repo.get_project_notes(&project.id).unwrap();
        assert_eq!(notes.len(), 0);
    }

    // Milestone Notes tests

    #[test]
    fn test_add_milestone_note() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let milestone = Milestone::new(project.id, 1, "Milestone 1".to_string());
        repo.add_milestone(&milestone).unwrap();

        let note = MilestoneNote::new(milestone.id, "Test Note".to_string(), "Note body".to_string());
        repo.add_milestone_note(&note).unwrap();

        let notes = repo.get_milestone_notes(&milestone.id).unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].title, "Test Note");
    }

    #[test]
    fn test_update_milestone_note() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let milestone = Milestone::new(project.id, 1, "Milestone 1".to_string());
        repo.add_milestone(&milestone).unwrap();

        let mut note = MilestoneNote::new(milestone.id, "Original Title".to_string(), "Original body".to_string());
        repo.add_milestone_note(&note).unwrap();

        note.title = "Updated Title".to_string();
        repo.update_milestone_note(&note).unwrap();

        let notes = repo.get_milestone_notes(&milestone.id).unwrap();
        assert_eq!(notes[0].title, "Updated Title");
    }

    #[test]
    fn test_delete_milestone_note() {
        let conn = setup_test_db();
        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let milestone = Milestone::new(project.id, 1, "Milestone 1".to_string());
        repo.add_milestone(&milestone).unwrap();

        let note = MilestoneNote::new(milestone.id, "Test Note".to_string(), "Note body".to_string());
        repo.add_milestone_note(&note).unwrap();

        repo.delete_milestone_note(&note.id).unwrap();

        let notes = repo.get_milestone_notes(&milestone.id).unwrap();
        assert_eq!(notes.len(), 0);
    }

    // Stakeholder Notes tests

    #[test]
    fn test_add_stakeholder_note() {
        let conn = setup_test_db();
        let person_repo = crate::db::PersonRepository::new(&conn);
        let repo = ProjectRepository::new(&conn);

        let person = crate::db::Person::new("alice@example.com".to_string(), "Alice".to_string());
        person_repo.create(&person).unwrap();

        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let stakeholder = ProjectStakeholder::new(project.id, "alice@example.com".to_string());
        repo.add_stakeholder(&project.id, &stakeholder).unwrap();

        let note = StakeholderNote::new(
            project.id,
            "alice@example.com".to_string(),
            "Test Note".to_string(),
            "Note body".to_string(),
        );
        repo.add_stakeholder_note(&note).unwrap();

        let notes = repo.get_stakeholder_notes(&project.id, "alice@example.com").unwrap();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].title, "Test Note");
    }

    #[test]
    fn test_update_stakeholder_note() {
        let conn = setup_test_db();
        let person_repo = crate::db::PersonRepository::new(&conn);
        let repo = ProjectRepository::new(&conn);

        let person = crate::db::Person::new("alice@example.com".to_string(), "Alice".to_string());
        person_repo.create(&person).unwrap();

        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let stakeholder = ProjectStakeholder::new(project.id, "alice@example.com".to_string());
        repo.add_stakeholder(&project.id, &stakeholder).unwrap();

        let mut note = StakeholderNote::new(
            project.id,
            "alice@example.com".to_string(),
            "Original Title".to_string(),
            "Original body".to_string(),
        );
        repo.add_stakeholder_note(&note).unwrap();

        note.title = "Updated Title".to_string();
        repo.update_stakeholder_note(&note).unwrap();

        let notes = repo.get_stakeholder_notes(&project.id, "alice@example.com").unwrap();
        assert_eq!(notes[0].title, "Updated Title");
    }

    #[test]
    fn test_delete_stakeholder_note() {
        let conn = setup_test_db();
        let person_repo = crate::db::PersonRepository::new(&conn);
        let repo = ProjectRepository::new(&conn);

        let person = crate::db::Person::new("alice@example.com".to_string(), "Alice".to_string());
        person_repo.create(&person).unwrap();

        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let stakeholder = ProjectStakeholder::new(project.id, "alice@example.com".to_string());
        repo.add_stakeholder(&project.id, &stakeholder).unwrap();

        let note = StakeholderNote::new(
            project.id,
            "alice@example.com".to_string(),
            "Test Note".to_string(),
            "Note body".to_string(),
        );
        repo.add_stakeholder_note(&note).unwrap();

        repo.delete_stakeholder_note(&note.id).unwrap();

        let notes = repo.get_stakeholder_notes(&project.id, "alice@example.com").unwrap();
        assert_eq!(notes.len(), 0);
    }
}
