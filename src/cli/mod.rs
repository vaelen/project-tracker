// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! CLI command handlers

use clap::Subcommand;
use project_tracker::{Config, Result};

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

pub async fn handle_projects(_action: ProjectAction, config: &Config) -> Result<()> {
    log::debug!("Data directory: {}", config.data_dir);
    println!("Project management - coming soon");
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
