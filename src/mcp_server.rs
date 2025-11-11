// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! MCP Server for Project Tracker (stdio transport)
//!
//! This server exposes all Project Tracker functionality via the Model Context Protocol
//! using stdio transport for integration with Claude Desktop and other AI assistants.

use anyhow::Result;
use project_tracker::{db, mcp::ProjectTrackerServer, Config};
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to stderr (stdout is used for MCP protocol)
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stderr)
        .init();

    log::info!("Starting Project Tracker MCP server (stdio transport)");

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
