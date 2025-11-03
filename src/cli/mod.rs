// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! CLI command handlers

use clap::Subcommand;
use claude_tracker::{Config, Result};

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

pub async fn handle_report(_format: &str, config: &Config) -> Result<()> {
    log::debug!("Data directory: {}", config.data_dir);
    println!("Report generation - coming soon");
    Ok(())
}
