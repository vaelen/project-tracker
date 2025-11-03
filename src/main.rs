// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! Claude Tracker CLI
//!
//! Command-line interface for Claude Tracker.

use clap::{Parser, Subcommand};
use claude_tracker::Result;

mod cli;

#[derive(Parser)]
#[command(name = "track")]
#[command(author = "Andrew C. Young <andrew@vaelen.org>")]
#[command(version)]
#[command(about = "Claude Tracker - Intelligent project and resource management", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage projects
    Projects {
        #[command(subcommand)]
        action: cli::ProjectAction,
    },
    /// Manage employees
    Employees {
        #[command(subcommand)]
        action: cli::EmployeeAction,
    },
    /// Manage deadlines
    Deadlines {
        #[command(subcommand)]
        action: cli::DeadlineAction,
    },
    /// Manage initiatives
    Initiatives {
        #[command(subcommand)]
        action: cli::InitiativeAction,
    },
    /// Manage stakeholders
    Stakeholders {
        #[command(subcommand)]
        action: cli::StakeholderAction,
    },
    /// Generate reports
    Report {
        /// Output format (markdown, text, json)
        #[arg(short, long, default_value = "markdown")]
        format: String,
    },
    /// Interactive mode with Claude AI
    Chat,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Projects { action } => cli::handle_projects(action).await?,
        Commands::Employees { action } => cli::handle_employees(action).await?,
        Commands::Deadlines { action } => cli::handle_deadlines(action).await?,
        Commands::Initiatives { action } => cli::handle_initiatives(action).await?,
        Commands::Stakeholders { action } => cli::handle_stakeholders(action).await?,
        Commands::Report { format } => cli::handle_report(&format).await?,
        Commands::Chat => cli::handle_chat().await?,
    }

    Ok(())
}
