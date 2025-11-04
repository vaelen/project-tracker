// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a person in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// Unique identifier (email address)
    pub email: String,

    /// Person's name
    pub name: String,

    /// Team name
    pub team: Option<String>,

    /// Manager's email address
    pub manager: Option<String>,

    /// Additional notes
    pub notes: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Person {
    /// Create a new person with required fields
    pub fn new(email: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            email,
            name,
            team: None,
            manager: None,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Represents a team in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    /// Unique identifier (team name)
    pub name: String,

    /// Team description
    pub description: Option<String>,

    /// Manager's email address
    pub manager: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Team {
    /// Create a new team with required fields
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            name,
            description: None,
            manager: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Represents a team member (junction table between teams and people)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    /// Team name
    pub team_name: String,

    /// Person's email address
    pub person_email: String,

    /// When this person was added to the team
    pub created_at: DateTime<Utc>,
}

impl TeamMember {
    /// Create a new team member relationship
    pub fn new(team_name: String, person_email: String) -> Self {
        Self {
            team_name,
            person_email,
            created_at: Utc::now(),
        }
    }
}

/// Represents a project milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    /// Unique identifier
    pub id: Uuid,

    /// Project this milestone belongs to
    pub project_id: Uuid,

    /// Milestone number (for ordering)
    pub number: i32,

    /// Milestone name
    pub name: String,

    /// Milestone description
    pub description: Option<String>,

    /// Technical lead email
    pub technical_lead: Option<String>,

    /// Team assigned to this milestone
    pub team: Option<String>,

    /// Link to design document
    pub design_doc_url: Option<String>,

    /// Due date
    pub due_date: Option<DateTime<Utc>>,

    /// Jira epic ticket number (e.g., "PROJ-123")
    pub jira_epic: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Milestone {
    /// Create a new milestone
    pub fn new(project_id: Uuid, number: i32, name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project_id,
            number,
            name,
            description: None,
            technical_lead: None,
            team: None,
            design_doc_url: None,
            due_date: None,
            jira_epic: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Represents a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Unique identifier
    pub id: Uuid,

    /// Project name
    pub name: String,

    /// Project description
    pub description: Option<String>,

    /// Project type (Team, Company, Personal)
    #[serde(rename = "type")]
    pub project_type: String,

    /// Requirements owner email
    pub requirements_owner: Option<String>,

    /// Technical lead email
    pub technical_lead: Option<String>,

    /// Manager email
    pub manager: Option<String>,

    /// Team assigned to this project
    pub team: Option<String>,

    /// Due date
    pub due_date: Option<DateTime<Utc>>,

    /// Jira initiative ticket number (e.g., "PROJ-123")
    pub jira_initiative: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Project {
    /// Create a new project
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            project_type: "Personal".to_string(),
            requirements_owner: None,
            technical_lead: None,
            manager: None,
            team: None,
            due_date: None,
            jira_initiative: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Represents a project stakeholder relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStakeholder {
    /// Project ID
    pub project_id: Uuid,

    /// Stakeholder email
    pub stakeholder_email: String,

    /// Stakeholder role/relationship to project
    pub role: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl ProjectStakeholder {
    /// Create a new project stakeholder relationship
    pub fn new(project_id: Uuid, stakeholder_email: String) -> Self {
        Self {
            project_id,
            stakeholder_email,
            role: None,
            created_at: Utc::now(),
        }
    }
}

/// Represents a note attached to a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectNote {
    /// Unique identifier
    pub id: Uuid,

    /// Project this note belongs to
    pub project_id: Uuid,

    /// Note title
    pub title: String,

    /// Note body/content
    pub body: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl ProjectNote {
    /// Create a new project note
    pub fn new(project_id: Uuid, title: String, body: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project_id,
            title,
            body,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Represents a note attached to a milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneNote {
    /// Unique identifier
    pub id: Uuid,

    /// Milestone this note belongs to
    pub milestone_id: Uuid,

    /// Note title
    pub title: String,

    /// Note body/content
    pub body: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl MilestoneNote {
    /// Create a new milestone note
    pub fn new(milestone_id: Uuid, title: String, body: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            milestone_id,
            title,
            body,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Represents a note attached to a stakeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeholderNote {
    /// Unique identifier
    pub id: Uuid,

    /// Project ID
    pub project_id: Uuid,

    /// Stakeholder email
    pub stakeholder_email: String,

    /// Note title
    pub title: String,

    /// Note body/content
    pub body: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl StakeholderNote {
    /// Create a new stakeholder note
    pub fn new(project_id: Uuid, stakeholder_email: String, title: String, body: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            project_id,
            stakeholder_email,
            title,
            body,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Person model tests

    #[test]
    fn test_person_new() {
        let person = Person::new(
            "alice@example.com".to_string(),
            "Alice Smith".to_string(),
        );

        assert_eq!(person.email, "alice@example.com");
        assert_eq!(person.name, "Alice Smith");
        assert!(person.team.is_none());
        assert!(person.manager.is_none());
        assert!(person.notes.is_none());
        // Check timestamps are set
        assert!(person.created_at < Utc::now() + chrono::Duration::seconds(1));
        assert!(person.updated_at < Utc::now() + chrono::Duration::seconds(1));
    }

    #[test]
    fn test_person_with_optional_fields() {
        let mut person = Person::new(
            "alice@example.com".to_string(),
            "Alice Smith".to_string(),
        );
        person.team = Some("Engineering".to_string());
        person.manager = Some("manager@example.com".to_string());
        person.notes = Some("Team lead".to_string());

        assert_eq!(person.team, Some("Engineering".to_string()));
        assert_eq!(person.manager, Some("manager@example.com".to_string()));
        assert_eq!(person.notes, Some("Team lead".to_string()));
    }

    // Project model tests

    #[test]
    fn test_project_new() {
        let project = Project::new("Test Project".to_string());

        assert_eq!(project.name, "Test Project");
        assert!(project.description.is_none());
        assert!(project.jira_initiative.is_none());
        assert_eq!(project.project_type, "Personal");
        assert!(project.due_date.is_none());
        // Check timestamps are set
        assert!(project.created_at < Utc::now() + chrono::Duration::seconds(1));
        assert!(project.updated_at < Utc::now() + chrono::Duration::seconds(1));
    }

    #[test]
    fn test_project_with_all_fields() {
        let mut project = Project::new("Test Project".to_string());
        project.description = Some("A test project".to_string());
        project.project_type = "Team".to_string();
        project.requirements_owner = Some("owner@example.com".to_string());
        project.technical_lead = Some("lead@example.com".to_string());
        project.manager = Some("manager@example.com".to_string());
        project.due_date = Some(Utc::now());
        project.jira_initiative = Some("PROJ-123".to_string());

        assert_eq!(project.description, Some("A test project".to_string()));
        assert_eq!(project.project_type, "Team");
        assert!(project.requirements_owner.is_some());
        assert!(project.technical_lead.is_some());
        assert!(project.manager.is_some());
        assert!(project.due_date.is_some());
        assert_eq!(project.jira_initiative, Some("PROJ-123".to_string()));
    }

    #[test]
    fn test_project_unique_ids() {
        let project1 = Project::new("Project 1".to_string());
        let project2 = Project::new("Project 2".to_string());

        assert_ne!(project1.id, project2.id);
    }

    // Milestone model tests

    #[test]
    fn test_milestone_new() {
        let project_id = Uuid::new_v4();
        let milestone = Milestone::new(
            project_id,
            1,
            "Alpha Release".to_string(),
        );

        assert_eq!(milestone.project_id, project_id);
        assert_eq!(milestone.number, 1);
        assert_eq!(milestone.name, "Alpha Release");
        assert!(milestone.description.is_none());
        assert!(milestone.jira_epic.is_none());
        assert!(milestone.due_date.is_none());
        // Check timestamps are set
        assert!(milestone.created_at < Utc::now() + chrono::Duration::seconds(1));
        assert!(milestone.updated_at < Utc::now() + chrono::Duration::seconds(1));
    }

    #[test]
    fn test_milestone_with_optional_fields() {
        let project_id = Uuid::new_v4();
        let mut milestone = Milestone::new(
            project_id,
            1,
            "Alpha Release".to_string(),
        );
        milestone.description = Some("First major release".to_string());
        milestone.technical_lead = Some("lead@example.com".to_string());
        milestone.design_doc_url = Some("https://docs.example.com/design".to_string());
        milestone.due_date = Some(Utc::now());
        milestone.jira_epic = Some("EPIC-123".to_string());

        assert_eq!(milestone.description, Some("First major release".to_string()));
        assert!(milestone.technical_lead.is_some());
        assert!(milestone.design_doc_url.is_some());
        assert!(milestone.due_date.is_some());
        assert_eq!(milestone.jira_epic, Some("EPIC-123".to_string()));
    }

    // ProjectStakeholder model tests

    #[test]
    fn test_project_stakeholder_new() {
        let project_id = Uuid::new_v4();
        let stakeholder = ProjectStakeholder::new(
            project_id,
            "stakeholder@example.com".to_string(),
        );

        assert_eq!(stakeholder.project_id, project_id);
        assert_eq!(stakeholder.stakeholder_email, "stakeholder@example.com");
        assert!(stakeholder.role.is_none());
        assert!(stakeholder.created_at < Utc::now() + chrono::Duration::seconds(1));
    }

    #[test]
    fn test_project_stakeholder_with_role() {
        let project_id = Uuid::new_v4();
        let mut stakeholder = ProjectStakeholder::new(
            project_id,
            "stakeholder@example.com".to_string(),
        );
        stakeholder.role = Some("Product Owner".to_string());

        assert_eq!(stakeholder.role, Some("Product Owner".to_string()));
    }

    // ProjectNote model tests

    #[test]
    fn test_project_note_new() {
        let project_id = Uuid::new_v4();
        let note = ProjectNote::new(
            project_id,
            "Test Note".to_string(),
            "This is a test note.".to_string(),
        );

        assert_eq!(note.project_id, project_id);
        assert_eq!(note.title, "Test Note");
        assert_eq!(note.body, "This is a test note.");
        assert!(note.created_at < Utc::now() + chrono::Duration::seconds(1));
        assert!(note.updated_at < Utc::now() + chrono::Duration::seconds(1));
    }

    #[test]
    fn test_project_note_unique_ids() {
        let project_id = Uuid::new_v4();
        let note1 = ProjectNote::new(
            project_id,
            "Note 1".to_string(),
            "Body 1".to_string(),
        );
        let note2 = ProjectNote::new(
            project_id,
            "Note 2".to_string(),
            "Body 2".to_string(),
        );

        assert_ne!(note1.id, note2.id);
    }

    // MilestoneNote model tests

    #[test]
    fn test_milestone_note_new() {
        let milestone_id = Uuid::new_v4();
        let note = MilestoneNote::new(
            milestone_id,
            "Milestone Note".to_string(),
            "This is a milestone note.".to_string(),
        );

        assert_eq!(note.milestone_id, milestone_id);
        assert_eq!(note.title, "Milestone Note");
        assert_eq!(note.body, "This is a milestone note.");
        assert!(note.created_at < Utc::now() + chrono::Duration::seconds(1));
        assert!(note.updated_at < Utc::now() + chrono::Duration::seconds(1));
    }

    // StakeholderNote model tests

    #[test]
    fn test_stakeholder_note_new() {
        let project_id = Uuid::new_v4();
        let note = StakeholderNote::new(
            project_id,
            "stakeholder@example.com".to_string(),
            "Stakeholder Note".to_string(),
            "This is a stakeholder note.".to_string(),
        );

        assert_eq!(note.project_id, project_id);
        assert_eq!(note.stakeholder_email, "stakeholder@example.com");
        assert_eq!(note.title, "Stakeholder Note");
        assert_eq!(note.body, "This is a stakeholder note.");
        assert!(note.created_at < Utc::now() + chrono::Duration::seconds(1));
        assert!(note.updated_at < Utc::now() + chrono::Duration::seconds(1));
    }

    // Serialization tests

    #[test]
    fn test_person_serialization() {
        let person = Person::new(
            "alice@example.com".to_string(),
            "Alice Smith".to_string(),
        );

        let json = serde_json::to_string(&person).unwrap();
        let deserialized: Person = serde_json::from_str(&json).unwrap();

        assert_eq!(person.email, deserialized.email);
        assert_eq!(person.name, deserialized.name);
    }

    #[test]
    fn test_project_serialization() {
        let project = Project::new("Test Project".to_string());

        let json = serde_json::to_string(&project).unwrap();
        let deserialized: Project = serde_json::from_str(&json).unwrap();

        assert_eq!(project.id, deserialized.id);
        assert_eq!(project.name, deserialized.name);
    }
}
