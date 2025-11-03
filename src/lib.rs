// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! Project Tracker - Core Library
//!
//! This library provides the core functionality for Project Tracker,
//! a project and resource management application for engineering managers.

pub mod config;
pub mod core;
pub mod db;
pub mod storage;
pub mod utils;

// Re-export commonly used types
pub use config::Config;
pub use core::{Deadline, Employee, Initiative, Stakeholder};
pub use db::{Milestone, Person, Project};
pub use storage::Storage;

/// Result type used throughout the library
pub type Result<T> = anyhow::Result<T>;
