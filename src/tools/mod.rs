// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! Claude agent tool definitions
//!
//! Tools that Claude can use to interact with the tracker data

use crate::Result;

/// Tool for managing projects
pub struct ProjectTool;

impl ProjectTool {
    pub async fn list_projects() -> Result<Vec<String>> {
        // TODO: Implement project listing
        Ok(vec![])
    }

    pub async fn create_project(_name: &str) -> Result<String> {
        // TODO: Implement project creation
        Ok("project-id".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_projects() {
        let projects = ProjectTool::list_projects().await.unwrap();
        assert!(projects.is_empty());
    }
}
