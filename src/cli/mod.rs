// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! CLI command handlers

use clap::Subcommand;
use project_tracker::{Config, Result};
use project_tracker::db::{self, MilestoneResource, ProjectRepository, ProjectResource};
use chrono::Utc;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum ProjectAction {
    /// List all projects
    List,
    /// Add a new project
    Add { name: String },
    /// Remove a project
    Remove { id: String },
    /// Show project details
    Show { id: String },
    /// Add a resource to a project
    AddResource {
        project_id: String,
        person_email: String,
        #[arg(short, long)]
        role: Option<String>,
    },
    /// List resources for a project
    ListResources { project_id: String },
    /// Remove a resource from a project
    RemoveResource {
        project_id: String,
        person_email: String,
    },
    /// Add a resource to a milestone
    AddMilestoneResource {
        milestone_id: String,
        person_email: String,
        #[arg(short, long)]
        role: Option<String>,
    },
    /// List resources for a milestone
    ListMilestoneResources { milestone_id: String },
    /// Remove a resource from a milestone
    RemoveMilestoneResource {
        milestone_id: String,
        person_email: String,
    },
}

#[derive(Subcommand)]
pub enum PeopleAction {
    /// List all people
    List,
    /// Add a new person
    Add { name: String },
    /// Remove a person
    Remove { id: String },
    /// Show person details
    Show { id: String },
}

#[derive(Subcommand)]
pub enum TeamAction {
    /// List all teams
    List,
    /// Add a new team
    Add { name: String },
    /// Remove a team
    Remove { name: String },
    /// Show team details
    Show { name: String },
    /// Add a member to a team
    AddMember {
        team_name: String,
        person_email: String,
    },
    /// Remove a member from a team
    RemoveMember {
        team_name: String,
        person_email: String,
    },
}

pub async fn handle_projects(action: ProjectAction, config: &Config) -> Result<()> {
    log::debug!("Data directory: {}", config.data_dir);

    let db_path = config.database_path()?;
    let conn = db::open_database(&db_path)?;
    let repo = ProjectRepository::new(&conn);

    match action {
        ProjectAction::List => {
            println!("Project management - coming soon");
        }
        ProjectAction::Add { .. } => {
            println!("Project management - coming soon");
        }
        ProjectAction::Remove { .. } => {
            println!("Project management - coming soon");
        }
        ProjectAction::Show { .. } => {
            println!("Project management - coming soon");
        }
        ProjectAction::AddResource { project_id, person_email, role } => {
            let project_uuid = Uuid::parse_str(&project_id)?;
            let resource = ProjectResource {
                project_id: project_uuid,
                person_email: person_email.clone(),
                role,
                created_at: Utc::now(),
            };
            repo.add_project_resource(&project_uuid, &resource)?;
            println!("Added resource {} to project {}", person_email, project_id);
        }
        ProjectAction::ListResources { project_id } => {
            let project_uuid = Uuid::parse_str(&project_id)?;
            let resources = repo.get_project_resources(&project_uuid)?;

            if resources.is_empty() {
                println!("No resources found for project {}", project_id);
            } else {
                println!("Resources for project {}:", project_id);
                for resource in resources {
                    if let Some(role) = resource.role {
                        println!("  {} ({})", resource.person_email, role);
                    } else {
                        println!("  {}", resource.person_email);
                    }
                }
            }
        }
        ProjectAction::RemoveResource { project_id, person_email } => {
            let project_uuid = Uuid::parse_str(&project_id)?;
            repo.remove_project_resource(&project_uuid, &person_email)?;
            println!("Removed resource {} from project {}", person_email, project_id);
        }
        ProjectAction::AddMilestoneResource { milestone_id, person_email, role } => {
            let milestone_uuid = Uuid::parse_str(&milestone_id)?;
            let resource = MilestoneResource {
                milestone_id: milestone_uuid,
                person_email: person_email.clone(),
                role,
                created_at: Utc::now(),
            };
            repo.add_milestone_resource(&milestone_uuid, &resource)?;
            println!("Added resource {} to milestone {}", person_email, milestone_id);
        }
        ProjectAction::ListMilestoneResources { milestone_id } => {
            let milestone_uuid = Uuid::parse_str(&milestone_id)?;
            let resources = repo.get_milestone_resources(&milestone_uuid)?;

            if resources.is_empty() {
                println!("No resources found for milestone {}", milestone_id);
            } else {
                println!("Resources for milestone {}:", milestone_id);
                for resource in resources {
                    if let Some(role) = resource.role {
                        println!("  {} ({})", resource.person_email, role);
                    } else {
                        println!("  {}", resource.person_email);
                    }
                }
            }
        }
        ProjectAction::RemoveMilestoneResource { milestone_id, person_email } => {
            let milestone_uuid = Uuid::parse_str(&milestone_id)?;
            repo.remove_milestone_resource(&milestone_uuid, &person_email)?;
            println!("Removed resource {} from milestone {}", person_email, milestone_id);
        }
    }

    Ok(())
}

pub async fn handle_people(_action: PeopleAction, config: &Config) -> Result<()> {
    log::debug!("Data directory: {}", config.data_dir);
    println!("People management - coming soon");
    Ok(())
}

pub async fn handle_teams(_action: TeamAction, config: &Config) -> Result<()> {
    log::debug!("Data directory: {}", config.data_dir);
    println!("Team management - coming soon");
    Ok(())
}

pub async fn handle_report(_format: &str, config: &Config) -> Result<()> {
    log::debug!("Data directory: {}", config.data_dir);
    println!("Report generation - coming soon");
    Ok(())
}
