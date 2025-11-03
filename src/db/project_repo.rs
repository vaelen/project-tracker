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
                                  manager, due_date, jira_initiative, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                project.id.to_string(),
                &project.name,
                &project.description,
                &project.project_type,
                &project.requirements_owner,
                &project.technical_lead,
                &project.manager,
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
                "SELECT id, name, description, type, requirements_owner, technical_lead, manager,
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
                        due_date: row.get(7)?,
                        jira_initiative: row.get(8)?,
                        created_at: row.get(9)?,
                        updated_at: row.get(10)?,
                    })
                },
            )
            .optional()?;
        Ok(project)
    }

    /// List all projects
    pub fn list_all(&self) -> Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, type, requirements_owner, technical_lead, manager,
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
                    due_date: row.get(7)?,
                    jira_initiative: row.get(8)?,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(projects)
    }

    /// Update a project
    pub fn update(&self, project: &Project) -> Result<()> {
        let rows = self.conn.execute(
            "UPDATE projects SET name = ?1, description = ?2, type = ?3, requirements_owner = ?4,
                                technical_lead = ?5, manager = ?6, due_date = ?7,
                                jira_initiative = ?8, updated_at = ?9
             WHERE id = ?10",
            params![
                &project.name,
                &project.description,
                &project.project_type,
                &project.requirements_owner,
                &project.technical_lead,
                &project.manager,
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
            "SELECT id, project_id, number, name, description, technical_lead,
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
                    design_doc_url: row.get(6)?,
                    due_date: row.get(7)?,
                    jira_epic: row.get(8)?,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(milestones)
    }

    /// Add milestone to project
    pub fn add_milestone(&self, milestone: &Milestone) -> Result<()> {
        self.conn.execute(
            "INSERT INTO milestones (id, project_id, number, name, description, technical_lead,
                                    design_doc_url, due_date, jira_epic, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                milestone.id.to_string(),
                milestone.project_id.to_string(),
                milestone.number,
                &milestone.name,
                &milestone.description,
                &milestone.technical_lead,
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
                                   design_doc_url = ?5, due_date = ?6, jira_epic = ?7, updated_at = ?8
             WHERE id = ?9",
            params![
                milestone.number,
                &milestone.name,
                &milestone.description,
                &milestone.technical_lead,
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

    #[test]
    fn test_create_and_find_project() {
        let conn = Connection::open_in_memory().unwrap();
        db::schema::initialize_schema(&conn).unwrap();

        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());

        repo.create(&project).unwrap();

        let found = repo.find_by_id(&project.id).unwrap().unwrap();
        assert_eq!(found.name, "Test Project");
    }

    #[test]
    fn test_add_milestone() {
        let conn = Connection::open_in_memory().unwrap();
        db::schema::initialize_schema(&conn).unwrap();

        let repo = ProjectRepository::new(&conn);
        let project = Project::new("Test Project".to_string());
        repo.create(&project).unwrap();

        let milestone = Milestone::new(project.id, 1, "Milestone 1".to_string());
        repo.add_milestone(&milestone).unwrap();

        let milestones = repo.get_milestones(&project.id).unwrap();
        assert_eq!(milestones.len(), 1);
        assert_eq!(milestones[0].name, "Milestone 1");
    }
}
