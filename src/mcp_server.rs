// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! MCP Server for Project Tracker
//!
//! This server exposes all Project Tracker functionality via the Model Context Protocol,
//! allowing AI assistants to interact with projects, people, milestones, and notes.

use anyhow::Result;
use project_tracker::{db, Config};
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Shared application state
#[derive(Clone)]
struct ProjectTrackerServer {
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
struct GetMilestonesRequest {
    /// Project UUID
    project_id: String,
}

#[tool_router]
impl ProjectTrackerServer {
    fn new(config: Config, db: Connection) -> Self {
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
                "Project Tracker MCP Server. Available tools: list_projects, get_project, create_project, \
                list_people, search_people, get_person, create_person, list_milestones".to_string()
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

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to stderr (stdout is used for MCP protocol)
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stderr)
        .init();

    log::info!("Starting Project Tracker MCP server");

    // Load configuration
    let config = Config::load_or_default()?;
    config.ensure_data_dir()?;

    // Open database
    let db_path = config.database_path()?;
    let conn = db::open_database(&db_path)?;

    // Create server
    let server = ProjectTrackerServer::new(config, conn);

    // Serve via stdio
    server.serve(rmcp::transport::stdio()).await?.waiting().await?;

    Ok(())
}
