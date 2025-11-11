// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! MCP Server Module
//!
//! This module provides shared MCP server functionality that can be used
//! with different transports (stdio, HTTP/SSE).

pub mod server;

pub use server::ProjectTrackerServer;
