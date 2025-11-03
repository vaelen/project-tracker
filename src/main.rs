// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! Claude Tracker CLI
//!
//! Command-line interface for Claude Tracker.

use clap::{Parser, Subcommand};
use claude_tracker::{Config, Result};
use std::path::PathBuf;

mod cli;

#[derive(Parser)]
#[command(name = "track")]
#[command(author = "Andrew C. Young <andrew@vaelen.org>")]
#[command(version)]
#[command(about = "Claude Tracker - Intelligent project and resource management", long_about = None)]
struct Cli {
    /// Path to configuration file
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

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
    let cli = Cli::parse();

    // Load configuration
    let config = if let Some(config_path) = &cli.config {
        Config::load(config_path)?
    } else {
        Config::load_or_default()?
    };

    // Initialize logging with configured level
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(&config.logging.level)
    ).init();

    log::info!("Claude Tracker v{}", env!("CARGO_PKG_VERSION"));
    log::debug!("Config loaded from: {}",
        cli.config.as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| Config::default_path()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| "unknown".to_string())
            )
    );
    log::debug!("Data directory: {}", config.data_dir);

    // Ensure data directories exist
    config.ensure_data_dirs()?;

    match cli.command {
        Commands::Projects { action } => cli::handle_projects(action, &config).await?,
        Commands::Employees { action } => cli::handle_employees(action, &config).await?,
        Commands::Deadlines { action } => cli::handle_deadlines(action, &config).await?,
        Commands::Initiatives { action } => cli::handle_initiatives(action, &config).await?,
        Commands::Stakeholders { action } => cli::handle_stakeholders(action, &config).await?,
        Commands::Report { format } => cli::handle_report(&format, &config).await?,
        Commands::Chat => cli::handle_chat(&config).await?,
    }

    Ok(())
}
