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
    }

    #[test]
    fn test_project_new() {
        let project = Project::new("Test Project".to_string());

        assert_eq!(project.name, "Test Project");
        assert!(project.description.is_none());
        assert!(project.jira_initiative.is_none());
    }

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
        assert!(milestone.jira_epic.is_none());
    }
}
