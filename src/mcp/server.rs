// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! Shared MCP Server Implementation
//!
//! This module provides the core MCP server functionality that can be used
//! with different transports (stdio, HTTP/SSE).

use crate::{db, Config};
use anyhow::Result;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use rusqlite::{Connection, OptionalExtension};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Shared application state
#[derive(Clone)]
pub struct ProjectTrackerServer {
    db: Arc<Mutex<Connection>>,
    _config: Arc<Config>,
    tool_router: ToolRouter<Self>,
}

// Request/Response types for tools
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetProjectRequest {
    /// Project UUID
    id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct CreateProjectRequest {
    /// Project name
    name: String,
    /// Project description
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// Project type (Personal, Team, Company)
    #[serde(skip_serializing_if = "Option::is_none")]
    project_type: Option<String>,
    /// JIRA initiative ID
    #[serde(skip_serializing_if = "Option::is_none")]
    jira_initiative: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct SearchPeopleRequest {
    /// Search query
    query: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetPersonRequest {
    /// Person email
    email: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct CreatePersonRequest {
    /// Person email
    email: String,
    /// Person name
    name: String,
    /// Team name
    #[serde(skip_serializing_if = "Option::is_none")]
    team: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct SearchTeamsRequest {
    /// Search query
    query: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetTeamRequest {
    /// Team name
    name: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct CreateTeamRequest {
    /// Team name
    name: String,
    /// Team description
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// Manager email
    #[serde(skip_serializing_if = "Option::is_none")]
    manager: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct TeamMemberRequest {
    /// Team name
    team_name: String,
    /// Person email
    person_email: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetTeamMembersRequest {
    /// Team name
    team_name: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetMilestonesRequest {
    /// Project UUID
    project_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct AddProjectResourceRequest {
    /// Project UUID
    project_id: String,
    /// Person email
    person_email: String,
    /// Resource role
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetProjectResourcesRequest {
    /// Project UUID
    project_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct RemoveProjectResourceRequest {
    /// Project UUID
    project_id: String,
    /// Person email
    person_email: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct AddMilestoneResourceRequest {
    /// Milestone UUID
    milestone_id: String,
    /// Person email
    person_email: String,
    /// Resource role
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetMilestoneResourcesRequest {
    /// Milestone UUID
    milestone_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct RemoveMilestoneResourceRequest {
    /// Milestone UUID
    milestone_id: String,
    /// Person email
    person_email: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UpdatePersonRequest {
    /// Person email
    email: String,
    /// Person name
    name: String,
    /// Team name
    #[serde(skip_serializing_if = "Option::is_none")]
    team: Option<String>,
    /// Manager email
    #[serde(skip_serializing_if = "Option::is_none")]
    manager: Option<String>,
    /// Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct DeletePersonRequest {
    /// Person email
    email: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UpdateTeamRequest {
    /// Team name
    name: String,
    /// Team description
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// Manager email
    #[serde(skip_serializing_if = "Option::is_none")]
    manager: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct DeleteTeamRequest {
    /// Team name
    name: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UpdateProjectRequest {
    /// Project UUID
    id: String,
    /// Project name
    name: String,
    /// Project description
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// Project type (Personal, Team, Company)
    #[serde(skip_serializing_if = "Option::is_none")]
    project_type: Option<String>,
    /// Requirements owner email
    #[serde(skip_serializing_if = "Option::is_none")]
    requirements_owner: Option<String>,
    /// Technical lead email
    #[serde(skip_serializing_if = "Option::is_none")]
    technical_lead: Option<String>,
    /// Manager email
    #[serde(skip_serializing_if = "Option::is_none")]
    manager: Option<String>,
    /// Team name
    #[serde(skip_serializing_if = "Option::is_none")]
    team: Option<String>,
    /// Start date (RFC3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    /// Due date (RFC3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<String>,
    /// JIRA initiative ID
    #[serde(skip_serializing_if = "Option::is_none")]
    jira_initiative: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct DeleteProjectRequest {
    /// Project UUID
    id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetMilestoneRequest {
    /// Milestone UUID
    id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct CreateMilestoneRequest {
    /// Project UUID
    project_id: String,
    /// Milestone number (for ordering)
    number: i32,
    /// Milestone name
    name: String,
    /// Milestone description
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// Technical lead email
    #[serde(skip_serializing_if = "Option::is_none")]
    technical_lead: Option<String>,
    /// Team name
    #[serde(skip_serializing_if = "Option::is_none")]
    team: Option<String>,
    /// Design doc URL
    #[serde(skip_serializing_if = "Option::is_none")]
    design_doc_url: Option<String>,
    /// Start date (RFC3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    /// Due date (RFC3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<String>,
    /// JIRA epic ID
    #[serde(skip_serializing_if = "Option::is_none")]
    jira_epic: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UpdateMilestoneRequest {
    /// Milestone UUID
    id: String,
    /// Milestone number (for ordering)
    number: i32,
    /// Milestone name
    name: String,
    /// Milestone description
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// Technical lead email
    #[serde(skip_serializing_if = "Option::is_none")]
    technical_lead: Option<String>,
    /// Team name
    #[serde(skip_serializing_if = "Option::is_none")]
    team: Option<String>,
    /// Design doc URL
    #[serde(skip_serializing_if = "Option::is_none")]
    design_doc_url: Option<String>,
    /// Start date (RFC3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    /// Due date (RFC3339)
    #[serde(skip_serializing_if = "Option::is_none")]
    due_date: Option<String>,
    /// JIRA epic ID
    #[serde(skip_serializing_if = "Option::is_none")]
    jira_epic: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct DeleteMilestoneRequest {
    /// Milestone UUID
    id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct AddProjectStakeholderRequest {
    /// Project UUID
    project_id: String,
    /// Stakeholder email
    stakeholder_email: String,
    /// Stakeholder role
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetProjectStakeholdersRequest {
    /// Project UUID
    project_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UpdateProjectStakeholderRequest {
    /// Project UUID
    project_id: String,
    /// Stakeholder email
    stakeholder_email: String,
    /// Stakeholder role
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct RemoveProjectStakeholderRequest {
    /// Project UUID
    project_id: String,
    /// Stakeholder email
    stakeholder_email: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UpdateProjectResourceRequest {
    /// Project UUID
    project_id: String,
    /// Person email
    person_email: String,
    /// Resource role
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UpdateMilestoneResourceRequest {
    /// Milestone UUID
    milestone_id: String,
    /// Person email
    person_email: String,
    /// Resource role
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct CreateProjectNoteRequest {
    /// Project UUID
    project_id: String,
    /// Note title
    title: String,
    /// Note body
    body: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetProjectNotesRequest {
    /// Project UUID
    project_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UpdateProjectNoteRequest {
    /// Note UUID
    id: String,
    /// Note title
    title: String,
    /// Note body
    body: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct DeleteProjectNoteRequest {
    /// Note UUID
    id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct CreateMilestoneNoteRequest {
    /// Milestone UUID
    milestone_id: String,
    /// Note title
    title: String,
    /// Note body
    body: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetMilestoneNotesRequest {
    /// Milestone UUID
    milestone_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UpdateMilestoneNoteRequest {
    /// Note UUID
    id: String,
    /// Note title
    title: String,
    /// Note body
    body: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct DeleteMilestoneNoteRequest {
    /// Note UUID
    id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct CreateStakeholderNoteRequest {
    /// Project UUID
    project_id: String,
    /// Stakeholder email
    stakeholder_email: String,
    /// Note title
    title: String,
    /// Note body
    body: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct GetStakeholderNotesRequest {
    /// Project UUID
    project_id: String,
    /// Stakeholder email
    stakeholder_email: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct UpdateStakeholderNoteRequest {
    /// Note UUID
    id: String,
    /// Note title
    title: String,
    /// Note body
    body: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct DeleteStakeholderNoteRequest {
    /// Note UUID
    id: String,
}

#[tool_router]
impl ProjectTrackerServer {
    pub fn new(config: Config, db: Connection) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            _config: Arc::new(config),
            tool_router: Self::tool_router(),
        }
    }

    // Project tools

    #[tool(description = "List all projects")]
    async fn list_projects(&self) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        let projects = repo.list_all().map_err(|e| {
            McpError::internal_error("Failed to list projects", Some(serde_json::json!({"error": e.to_string()})))
        })?;

        let json = serde_json::to_string_pretty(&projects)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get a project by ID")]
    async fn get_project(&self, Parameters(req): Parameters<GetProjectRequest>) -> Result<CallToolResult, McpError> {
        let uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        let project = repo.find_by_id(&uuid)
            .map_err(|e| McpError::internal_error("Database error", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&project)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Create a new project")]
    async fn create_project(&self, Parameters(req): Parameters<CreateProjectRequest>) -> Result<CallToolResult, McpError> {
        let mut project = db::Project::new(req.name);

        if let Some(desc) = req.description {
            project.description = Some(desc);
        }
        if let Some(ptype) = req.project_type {
            project.project_type = ptype;
        }
        if let Some(jira) = req.jira_initiative {
            project.jira_initiative = Some(jira);
        }

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.create(&project)
            .map_err(|e| McpError::internal_error("Failed to create project", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&project)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Update a project")]
    async fn update_project(&self, Parameters(req): Parameters<UpdateProjectRequest>) -> Result<CallToolResult, McpError> {
        let uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);

        // Fetch existing project first
        let mut project = repo.find_by_id(&uuid)
            .map_err(|e| McpError::internal_error("Database error", Some(serde_json::json!({"error": e.to_string()}))))?
            .ok_or_else(|| McpError::invalid_params("Project not found", None))?;

        // Update fields
        project.name = req.name;
        project.description = req.description;
        if let Some(ptype) = req.project_type {
            project.project_type = ptype;
        }
        project.requirements_owner = req.requirements_owner;
        project.technical_lead = req.technical_lead;
        project.manager = req.manager;
        project.team = req.team;
        project.jira_initiative = req.jira_initiative;

        // Parse dates if provided
        if let Some(start_date_str) = req.start_date {
            project.start_date = Some(chrono::DateTime::parse_from_rfc3339(&start_date_str)
                .map_err(|e| McpError::invalid_params("Invalid start_date format", Some(serde_json::json!({"error": e.to_string()}))))?
                .with_timezone(&chrono::Utc));
        }
        if let Some(due_date_str) = req.due_date {
            project.due_date = Some(chrono::DateTime::parse_from_rfc3339(&due_date_str)
                .map_err(|e| McpError::invalid_params("Invalid due_date format", Some(serde_json::json!({"error": e.to_string()}))))?
                .with_timezone(&chrono::Utc));
        }

        repo.update(&project)
            .map_err(|e| McpError::internal_error("Failed to update project", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&project)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Delete a project")]
    async fn delete_project(&self, Parameters(req): Parameters<DeleteProjectRequest>) -> Result<CallToolResult, McpError> {
        let uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.delete(&uuid)
            .map_err(|e| McpError::internal_error("Failed to delete project", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Deleted project {}", req.id))]))
    }

    // People tools

    #[tool(description = "List all people")]
    async fn list_people(&self) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::PersonRepository::new(&db);
        let people = repo.list_all()
            .map_err(|e| McpError::internal_error("Failed to list people", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&people)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Search people by name")]
    async fn search_people(&self, Parameters(req): Parameters<SearchPeopleRequest>) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::PersonRepository::new(&db);
        let people = repo.search_by_name(&req.query)
            .map_err(|e| McpError::internal_error("Search failed", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&people)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get a person by email")]
    async fn get_person(&self, Parameters(req): Parameters<GetPersonRequest>) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::PersonRepository::new(&db);
        let person = repo.find_by_email(&req.email)
            .map_err(|e| McpError::internal_error("Database error", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&person)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Create a new person")]
    async fn create_person(&self, Parameters(req): Parameters<CreatePersonRequest>) -> Result<CallToolResult, McpError> {
        let mut person = db::Person::new(req.email, req.name);

        if let Some(team) = req.team {
            person.team = Some(team);
        }

        let db = self.db.lock().await;
        let repo = db::PersonRepository::new(&db);
        repo.create(&person)
            .map_err(|e| McpError::internal_error("Failed to create person", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&person)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Update a person")]
    async fn update_person(&self, Parameters(req): Parameters<UpdatePersonRequest>) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::PersonRepository::new(&db);

        // Fetch existing person first
        let mut person = repo.find_by_email(&req.email)
            .map_err(|e| McpError::internal_error("Database error", Some(serde_json::json!({"error": e.to_string()}))))?
            .ok_or_else(|| McpError::invalid_params("Person not found", None))?;

        // Update fields
        person.name = req.name;
        person.team = req.team;
        person.manager = req.manager;
        person.notes = req.notes;

        repo.update(&person)
            .map_err(|e| McpError::internal_error("Failed to update person", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&person)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Delete a person")]
    async fn delete_person(&self, Parameters(req): Parameters<DeletePersonRequest>) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::PersonRepository::new(&db);
        repo.delete(&req.email)
            .map_err(|e| McpError::internal_error("Failed to delete person", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Deleted person {}", req.email))]))
    }

    // Team tools

    #[tool(description = "List all teams")]
    async fn list_teams(&self) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::TeamRepository::new(&db);
        let teams = repo.list_all()
            .map_err(|e| McpError::internal_error("Failed to list teams", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&teams)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Search teams by name")]
    async fn search_teams(&self, Parameters(req): Parameters<SearchTeamsRequest>) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::TeamRepository::new(&db);
        let teams = repo.search_by_name(&req.query)
            .map_err(|e| McpError::internal_error("Search failed", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&teams)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get a team by name")]
    async fn get_team(&self, Parameters(req): Parameters<GetTeamRequest>) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::TeamRepository::new(&db);
        let team = repo.find_by_name(&req.name)
            .map_err(|e| McpError::internal_error("Database error", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&team)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Create a new team")]
    async fn create_team(&self, Parameters(req): Parameters<CreateTeamRequest>) -> Result<CallToolResult, McpError> {
        let mut team = db::Team::new(req.name);

        if let Some(desc) = req.description {
            team.description = Some(desc);
        }
        if let Some(manager) = req.manager {
            team.manager = Some(manager);
        }

        let db = self.db.lock().await;
        let repo = db::TeamRepository::new(&db);
        repo.create(&team)
            .map_err(|e| McpError::internal_error("Failed to create team", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&team)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Update a team")]
    async fn update_team(&self, Parameters(req): Parameters<UpdateTeamRequest>) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::TeamRepository::new(&db);

        // Fetch existing team first
        let mut team = repo.find_by_name(&req.name)
            .map_err(|e| McpError::internal_error("Database error", Some(serde_json::json!({"error": e.to_string()}))))?
            .ok_or_else(|| McpError::invalid_params("Team not found", None))?;

        // Update fields
        team.description = req.description;
        team.manager = req.manager;

        repo.update(&team)
            .map_err(|e| McpError::internal_error("Failed to update team", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&team)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Delete a team")]
    async fn delete_team(&self, Parameters(req): Parameters<DeleteTeamRequest>) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::TeamRepository::new(&db);
        repo.delete(&req.name)
            .map_err(|e| McpError::internal_error("Failed to delete team", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Deleted team {}", req.name))]))
    }

    #[tool(description = "Add a member to a team")]
    async fn add_team_member(&self, Parameters(req): Parameters<TeamMemberRequest>) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::TeamRepository::new(&db);
        repo.add_member(&req.team_name, &req.person_email)
            .map_err(|e| McpError::internal_error("Failed to add team member", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Added {} to team {}", req.person_email, req.team_name))]))
    }

    #[tool(description = "Remove a member from a team")]
    async fn remove_team_member(&self, Parameters(req): Parameters<TeamMemberRequest>) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::TeamRepository::new(&db);
        repo.remove_member(&req.team_name, &req.person_email)
            .map_err(|e| McpError::internal_error("Failed to remove team member", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Removed {} from team {}", req.person_email, req.team_name))]))
    }

    #[tool(description = "Get all members of a team")]
    async fn get_team_members(&self, Parameters(req): Parameters<GetTeamMembersRequest>) -> Result<CallToolResult, McpError> {
        let db = self.db.lock().await;
        let repo = db::TeamRepository::new(&db);
        let members = repo.get_members(&req.team_name)
            .map_err(|e| McpError::internal_error("Failed to get team members", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&members)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    // Milestone tools

    #[tool(description = "List milestones for a project")]
    async fn list_milestones(&self, Parameters(req): Parameters<GetMilestonesRequest>) -> Result<CallToolResult, McpError> {
        let uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        let milestones = repo.get_milestones(&uuid)
            .map_err(|e| McpError::internal_error("Failed to list milestones", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&milestones)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get a milestone by ID")]
    async fn get_milestone(&self, Parameters(req): Parameters<GetMilestoneRequest>) -> Result<CallToolResult, McpError> {
        let milestone_uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;

        // Query milestone directly from database
        let milestone = db.query_row(
            "SELECT id, project_id, number, name, description, technical_lead, team, design_doc_url, start_date, due_date, jira_epic, created_at, updated_at
             FROM milestones WHERE id = ?1",
            rusqlite::params![milestone_uuid.to_string()],
            |row| {
                Ok(db::Milestone {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    project_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    number: row.get(2)?,
                    name: row.get(3)?,
                    description: row.get(4)?,
                    technical_lead: row.get(5)?,
                    team: row.get(6)?,
                    design_doc_url: row.get(7)?,
                    start_date: row.get(8)?,
                    due_date: row.get(9)?,
                    jira_epic: row.get(10)?,
                    created_at: row.get(11)?,
                    updated_at: row.get(12)?,
                })
            },
        ).optional()
            .map_err(|e| McpError::internal_error("Database error", Some(serde_json::json!({"error": e.to_string()}))))?
            .ok_or_else(|| McpError::invalid_params("Milestone not found", None))?;

        let json = serde_json::to_string_pretty(&milestone)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Create a new milestone")]
    async fn create_milestone(&self, Parameters(req): Parameters<CreateMilestoneRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid project UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let mut milestone = db::Milestone::new(project_uuid, req.number, req.name);
        milestone.description = req.description;
        milestone.technical_lead = req.technical_lead;
        milestone.team = req.team;
        milestone.design_doc_url = req.design_doc_url;
        milestone.jira_epic = req.jira_epic;

        // Parse dates if provided
        if let Some(start_date_str) = req.start_date {
            milestone.start_date = Some(chrono::DateTime::parse_from_rfc3339(&start_date_str)
                .map_err(|e| McpError::invalid_params("Invalid start_date format", Some(serde_json::json!({"error": e.to_string()}))))?
                .with_timezone(&chrono::Utc));
        }
        if let Some(due_date_str) = req.due_date {
            milestone.due_date = Some(chrono::DateTime::parse_from_rfc3339(&due_date_str)
                .map_err(|e| McpError::invalid_params("Invalid due_date format", Some(serde_json::json!({"error": e.to_string()}))))?
                .with_timezone(&chrono::Utc));
        }

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.add_milestone(&milestone)
            .map_err(|e| McpError::internal_error("Failed to create milestone", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&milestone)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Update a milestone")]
    async fn update_milestone(&self, Parameters(req): Parameters<UpdateMilestoneRequest>) -> Result<CallToolResult, McpError> {
        let milestone_uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;

        // Fetch existing milestone first - query directly
        let mut milestone = db.query_row(
            "SELECT id, project_id, number, name, description, technical_lead, team, design_doc_url, start_date, due_date, jira_epic, created_at, updated_at
             FROM milestones WHERE id = ?1",
            rusqlite::params![milestone_uuid.to_string()],
            |row| {
                Ok(db::Milestone {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    project_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    number: row.get(2)?,
                    name: row.get(3)?,
                    description: row.get(4)?,
                    technical_lead: row.get(5)?,
                    team: row.get(6)?,
                    design_doc_url: row.get(7)?,
                    start_date: row.get(8)?,
                    due_date: row.get(9)?,
                    jira_epic: row.get(10)?,
                    created_at: row.get(11)?,
                    updated_at: row.get(12)?,
                })
            },
        ).optional()
            .map_err(|e| McpError::internal_error("Database error", Some(serde_json::json!({"error": e.to_string()}))))?
            .ok_or_else(|| McpError::invalid_params("Milestone not found", None))?;

        // Update fields
        milestone.number = req.number;
        milestone.name = req.name;
        milestone.description = req.description;
        milestone.technical_lead = req.technical_lead;
        milestone.team = req.team;
        milestone.design_doc_url = req.design_doc_url;
        milestone.jira_epic = req.jira_epic;

        // Parse dates if provided
        if let Some(start_date_str) = req.start_date {
            milestone.start_date = Some(chrono::DateTime::parse_from_rfc3339(&start_date_str)
                .map_err(|e| McpError::invalid_params("Invalid start_date format", Some(serde_json::json!({"error": e.to_string()}))))?
                .with_timezone(&chrono::Utc));
        }
        if let Some(due_date_str) = req.due_date {
            milestone.due_date = Some(chrono::DateTime::parse_from_rfc3339(&due_date_str)
                .map_err(|e| McpError::invalid_params("Invalid due_date format", Some(serde_json::json!({"error": e.to_string()}))))?
                .with_timezone(&chrono::Utc));
        }

        let repo = db::ProjectRepository::new(&db);
        repo.update_milestone(&milestone)
            .map_err(|e| McpError::internal_error("Failed to update milestone", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&milestone)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Delete a milestone")]
    async fn delete_milestone(&self, Parameters(req): Parameters<DeleteMilestoneRequest>) -> Result<CallToolResult, McpError> {
        let milestone_uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.delete_milestone(&milestone_uuid)
            .map_err(|e| McpError::internal_error("Failed to delete milestone", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Deleted milestone {}", req.id))]))
    }

    // Project Stakeholder tools

    #[tool(description = "Add a stakeholder to a project")]
    async fn add_project_stakeholder(&self, Parameters(req): Parameters<AddProjectStakeholderRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let stakeholder = db::ProjectStakeholder {
            project_id: project_uuid,
            stakeholder_email: req.stakeholder_email.clone(),
            role: req.role,
            created_at: chrono::Utc::now(),
        };

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.add_stakeholder(&project_uuid, &stakeholder)
            .map_err(|e| McpError::internal_error("Failed to add stakeholder", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&stakeholder)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List stakeholders for a project")]
    async fn list_project_stakeholders(&self, Parameters(req): Parameters<GetProjectStakeholdersRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        let stakeholders = repo.get_stakeholders(&project_uuid)
            .map_err(|e| McpError::internal_error("Failed to list stakeholders", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&stakeholders)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Update a project stakeholder")]
    async fn update_project_stakeholder(&self, Parameters(req): Parameters<UpdateProjectStakeholderRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let stakeholder = db::ProjectStakeholder {
            project_id: project_uuid,
            stakeholder_email: req.stakeholder_email.clone(),
            role: req.role,
            created_at: chrono::Utc::now(), // This will be ignored by update
        };

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.update_stakeholder(&project_uuid, &stakeholder)
            .map_err(|e| McpError::internal_error("Failed to update stakeholder", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&stakeholder)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Remove a stakeholder from a project")]
    async fn remove_project_stakeholder(&self, Parameters(req): Parameters<RemoveProjectStakeholderRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.remove_stakeholder(&project_uuid, &req.stakeholder_email)
            .map_err(|e| McpError::internal_error("Failed to remove stakeholder", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Removed stakeholder {} from project {}", req.stakeholder_email, req.project_id))]))
    }

    // Project Resource tools

    #[tool(description = "Add a resource to a project")]
    async fn add_project_resource(&self, Parameters(req): Parameters<AddProjectResourceRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let resource = db::ProjectResource {
            project_id: project_uuid,
            person_email: req.person_email.clone(),
            role: req.role,
            created_at: chrono::Utc::now(),
        };

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.add_project_resource(&project_uuid, &resource)
            .map_err(|e| McpError::internal_error("Failed to add resource", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&resource)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List resources for a project")]
    async fn list_project_resources(&self, Parameters(req): Parameters<GetProjectResourcesRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        let resources = repo.get_project_resources(&project_uuid)
            .map_err(|e| McpError::internal_error("Failed to list resources", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&resources)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Update a project resource")]
    async fn update_project_resource(&self, Parameters(req): Parameters<UpdateProjectResourceRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let resource = db::ProjectResource {
            project_id: project_uuid,
            person_email: req.person_email.clone(),
            role: req.role,
            created_at: chrono::Utc::now(), // This will be ignored by update
        };

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.update_project_resource(&project_uuid, &resource)
            .map_err(|e| McpError::internal_error("Failed to update resource", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&resource)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Remove a resource from a project")]
    async fn remove_project_resource(&self, Parameters(req): Parameters<RemoveProjectResourceRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.remove_project_resource(&project_uuid, &req.person_email)
            .map_err(|e| McpError::internal_error("Failed to remove resource", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Removed resource {} from project {}", req.person_email, req.project_id))]))
    }

    // Milestone Resource tools

    #[tool(description = "Add a resource to a milestone")]
    async fn add_milestone_resource(&self, Parameters(req): Parameters<AddMilestoneResourceRequest>) -> Result<CallToolResult, McpError> {
        let milestone_uuid = Uuid::parse_str(&req.milestone_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let resource = db::MilestoneResource {
            milestone_id: milestone_uuid,
            person_email: req.person_email.clone(),
            role: req.role,
            created_at: chrono::Utc::now(),
        };

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.add_milestone_resource(&milestone_uuid, &resource)
            .map_err(|e| McpError::internal_error("Failed to add resource", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&resource)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List resources for a milestone")]
    async fn list_milestone_resources(&self, Parameters(req): Parameters<GetMilestoneResourcesRequest>) -> Result<CallToolResult, McpError> {
        let milestone_uuid = Uuid::parse_str(&req.milestone_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        let resources = repo.get_milestone_resources(&milestone_uuid)
            .map_err(|e| McpError::internal_error("Failed to list resources", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&resources)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Update a milestone resource")]
    async fn update_milestone_resource(&self, Parameters(req): Parameters<UpdateMilestoneResourceRequest>) -> Result<CallToolResult, McpError> {
        let milestone_uuid = Uuid::parse_str(&req.milestone_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let resource = db::MilestoneResource {
            milestone_id: milestone_uuid,
            person_email: req.person_email.clone(),
            role: req.role,
            created_at: chrono::Utc::now(), // This will be ignored by update
        };

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.update_milestone_resource(&milestone_uuid, &resource)
            .map_err(|e| McpError::internal_error("Failed to update resource", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&resource)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Remove a resource from a milestone")]
    async fn remove_milestone_resource(&self, Parameters(req): Parameters<RemoveMilestoneResourceRequest>) -> Result<CallToolResult, McpError> {
        let milestone_uuid = Uuid::parse_str(&req.milestone_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.remove_milestone_resource(&milestone_uuid, &req.person_email)
            .map_err(|e| McpError::internal_error("Failed to remove resource", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Removed resource {} from milestone {}", req.person_email, req.milestone_id))]))
    }

    // Project Note tools

    #[tool(description = "Create a note for a project")]
    async fn create_project_note(&self, Parameters(req): Parameters<CreateProjectNoteRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let note = db::ProjectNote::new(project_uuid, req.title, req.body);

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.add_project_note(&note)
            .map_err(|e| McpError::internal_error("Failed to create note", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&note)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List notes for a project")]
    async fn list_project_notes(&self, Parameters(req): Parameters<GetProjectNotesRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        let notes = repo.get_project_notes(&project_uuid)
            .map_err(|e| McpError::internal_error("Failed to list notes", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&notes)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Update a project note")]
    async fn update_project_note(&self, Parameters(req): Parameters<UpdateProjectNoteRequest>) -> Result<CallToolResult, McpError> {
        let note_uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;

        // Fetch existing note first
        let mut note = db.query_row(
            "SELECT id, project_id, title, body, created_at, updated_at FROM project_notes WHERE id = ?1",
            rusqlite::params![note_uuid.to_string()],
            |row| {
                Ok(db::ProjectNote {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    project_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    title: row.get(2)?,
                    body: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        ).optional()
            .map_err(|e| McpError::internal_error("Database error", Some(serde_json::json!({"error": e.to_string()}))))?
            .ok_or_else(|| McpError::invalid_params("Note not found", None))?;

        // Update fields
        note.title = req.title;
        note.body = req.body;
        note.updated_at = chrono::Utc::now();

        let repo = db::ProjectRepository::new(&db);
        repo.update_project_note(&note)
            .map_err(|e| McpError::internal_error("Failed to update note", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&note)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Delete a project note")]
    async fn delete_project_note(&self, Parameters(req): Parameters<DeleteProjectNoteRequest>) -> Result<CallToolResult, McpError> {
        let note_uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.delete_project_note(&note_uuid)
            .map_err(|e| McpError::internal_error("Failed to delete note", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Deleted note {}", req.id))]))
    }

    // Milestone Note tools

    #[tool(description = "Create a note for a milestone")]
    async fn create_milestone_note(&self, Parameters(req): Parameters<CreateMilestoneNoteRequest>) -> Result<CallToolResult, McpError> {
        let milestone_uuid = Uuid::parse_str(&req.milestone_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let note = db::MilestoneNote::new(milestone_uuid, req.title, req.body);

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.add_milestone_note(&note)
            .map_err(|e| McpError::internal_error("Failed to create note", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&note)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List notes for a milestone")]
    async fn list_milestone_notes(&self, Parameters(req): Parameters<GetMilestoneNotesRequest>) -> Result<CallToolResult, McpError> {
        let milestone_uuid = Uuid::parse_str(&req.milestone_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        let notes = repo.get_milestone_notes(&milestone_uuid)
            .map_err(|e| McpError::internal_error("Failed to list notes", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&notes)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Update a milestone note")]
    async fn update_milestone_note(&self, Parameters(req): Parameters<UpdateMilestoneNoteRequest>) -> Result<CallToolResult, McpError> {
        let note_uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;

        // Fetch existing note first
        let mut note = db.query_row(
            "SELECT id, milestone_id, title, body, created_at, updated_at FROM milestone_notes WHERE id = ?1",
            rusqlite::params![note_uuid.to_string()],
            |row| {
                Ok(db::MilestoneNote {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    milestone_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    title: row.get(2)?,
                    body: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        ).optional()
            .map_err(|e| McpError::internal_error("Database error", Some(serde_json::json!({"error": e.to_string()}))))?
            .ok_or_else(|| McpError::invalid_params("Note not found", None))?;

        // Update fields
        note.title = req.title;
        note.body = req.body;
        note.updated_at = chrono::Utc::now();

        let repo = db::ProjectRepository::new(&db);
        repo.update_milestone_note(&note)
            .map_err(|e| McpError::internal_error("Failed to update note", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&note)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Delete a milestone note")]
    async fn delete_milestone_note(&self, Parameters(req): Parameters<DeleteMilestoneNoteRequest>) -> Result<CallToolResult, McpError> {
        let note_uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.delete_milestone_note(&note_uuid)
            .map_err(|e| McpError::internal_error("Failed to delete note", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Deleted note {}", req.id))]))
    }

    // Stakeholder Note tools

    #[tool(description = "Create a note for a stakeholder")]
    async fn create_stakeholder_note(&self, Parameters(req): Parameters<CreateStakeholderNoteRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let note = db::StakeholderNote::new(project_uuid, req.stakeholder_email, req.title, req.body);

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.add_stakeholder_note(&note)
            .map_err(|e| McpError::internal_error("Failed to create note", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&note)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List notes for a stakeholder")]
    async fn list_stakeholder_notes(&self, Parameters(req): Parameters<GetStakeholderNotesRequest>) -> Result<CallToolResult, McpError> {
        let project_uuid = Uuid::parse_str(&req.project_id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        let notes = repo.get_stakeholder_notes(&project_uuid, &req.stakeholder_email)
            .map_err(|e| McpError::internal_error("Failed to list notes", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&notes)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Update a stakeholder note")]
    async fn update_stakeholder_note(&self, Parameters(req): Parameters<UpdateStakeholderNoteRequest>) -> Result<CallToolResult, McpError> {
        let note_uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;

        // Fetch existing note first
        let mut note = db.query_row(
            "SELECT id, project_id, stakeholder_email, title, body, created_at, updated_at FROM stakeholder_notes WHERE id = ?1",
            rusqlite::params![note_uuid.to_string()],
            |row| {
                Ok(db::StakeholderNote {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    project_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    stakeholder_email: row.get(2)?,
                    title: row.get(3)?,
                    body: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            },
        ).optional()
            .map_err(|e| McpError::internal_error("Database error", Some(serde_json::json!({"error": e.to_string()}))))?
            .ok_or_else(|| McpError::invalid_params("Note not found", None))?;

        // Update fields
        note.title = req.title;
        note.body = req.body;
        note.updated_at = chrono::Utc::now();

        let repo = db::ProjectRepository::new(&db);
        repo.update_stakeholder_note(&note)
            .map_err(|e| McpError::internal_error("Failed to update note", Some(serde_json::json!({"error": e.to_string()}))))?;

        let json = serde_json::to_string_pretty(&note)
            .map_err(|e| McpError::internal_error("Failed to serialize", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Delete a stakeholder note")]
    async fn delete_stakeholder_note(&self, Parameters(req): Parameters<DeleteStakeholderNoteRequest>) -> Result<CallToolResult, McpError> {
        let note_uuid = Uuid::parse_str(&req.id)
            .map_err(|e| McpError::invalid_params("Invalid UUID", Some(serde_json::json!({"error": e.to_string()}))))?;

        let db = self.db.lock().await;
        let repo = db::ProjectRepository::new(&db);
        repo.delete_stakeholder_note(&note_uuid)
            .map_err(|e| McpError::internal_error("Failed to delete note", Some(serde_json::json!({"error": e.to_string()}))))?;

        Ok(CallToolResult::success(vec![Content::text(format!("Deleted note {}", req.id))]))
    }
}

#[tool_handler]
impl ServerHandler for ProjectTrackerServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Project Tracker MCP Server. Available tools:\n\
                Projects: list_projects, get_project, create_project, update_project, delete_project\n\
                People: list_people, search_people, get_person, create_person, update_person, delete_person\n\
                Teams: list_teams, search_teams, get_team, create_team, update_team, delete_team, add_team_member, remove_team_member, get_team_members\n\
                Milestones: list_milestones, get_milestone, create_milestone, update_milestone, delete_milestone\n\
                Stakeholders: add_project_stakeholder, list_project_stakeholders, update_project_stakeholder, remove_project_stakeholder\n\
                Project Resources: add_project_resource, list_project_resources, update_project_resource, remove_project_resource\n\
                Milestone Resources: add_milestone_resource, list_milestone_resources, update_milestone_resource, remove_milestone_resource\n\
                Project Notes: create_project_note, list_project_notes, update_project_note, delete_project_note\n\
                Milestone Notes: create_milestone_note, list_milestone_notes, update_milestone_note, delete_milestone_note\n\
                Stakeholder Notes: create_stakeholder_note, list_stakeholder_notes, update_stakeholder_note, delete_stakeholder_note".to_string()
            ),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}
