// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! Core domain models and business logic

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// A project being tracked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Planning,
    Active,
    OnHold,
    Completed,
    Cancelled,
}

/// An employee/team member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

/// A deadline or milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deadline {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub due_date: DateTime<Utc>,
    pub project_id: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

/// A corporate initiative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Initiative {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub quarter: String,
    pub created_at: DateTime<Utc>,
}

/// A project stakeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stakeholder {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub project_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Resource allocation - which employee works on which project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allocation {
    pub id: String,
    pub employee_id: String,
    pub project_id: String,
    pub percentage: u8, // 0-100
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Interaction note with a stakeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub id: String,
    pub stakeholder_id: String,
    pub date: DateTime<Utc>,
    pub note: String,
    pub created_at: DateTime<Utc>,
}
