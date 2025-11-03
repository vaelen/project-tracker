// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! Claude Tracker - Core Library
//!
//! This library provides the core functionality for Claude Tracker,
//! a project and resource management application for engineering managers.

pub mod agent;
pub mod core;
pub mod storage;
pub mod tools;
pub mod utils;

// Re-export commonly used types
pub use core::{Project, Employee, Deadline, Initiative, Stakeholder};
pub use storage::Storage;
pub use agent::ClaudeAgent;

/// Result type used throughout the library
pub type Result<T> = anyhow::Result<T>;
