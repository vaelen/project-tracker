// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

//! Claude API integration

use crate::Result;

/// Claude Agent for AI-powered assistance
pub struct ClaudeAgent {
    api_key: String,
}

impl ClaudeAgent {
    /// Create a new Claude agent instance
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    /// Send a message to Claude and get a response
    pub async fn chat(&self, _message: &str) -> Result<String> {
        // TODO: Implement Claude API integration
        Ok("Claude API integration - coming soon".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = ClaudeAgent::new("test-key".to_string());
        assert_eq!(agent.api_key, "test-key");
    }
}
