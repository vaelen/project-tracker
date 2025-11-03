// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::auth::Credentials;

/// Chat message role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

/// Claude API request
#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

/// Claude API response
#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    _type: String,
    text: String,
}

/// Claude Chat client
pub struct ChatClient {
    api_key: String,
    organization_id: Option<String>,
    client: reqwest::Client,
}

impl ChatClient {
    /// Create a new chat client with credentials
    pub fn new(credentials: &Credentials) -> Self {
        Self {
            api_key: credentials.session_key.clone(),
            organization_id: credentials.organization_id.clone(),
            client: reqwest::Client::new(),
        }
    }

    /// Send a message to Claude and get a response
    pub async fn send_message(&self, messages: Vec<Message>) -> Result<String> {
        let request = ClaudeRequest {
            model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 8192,
            messages,
        };

        let mut req = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json");

        // Add organization ID if present
        if let Some(org_id) = &self.organization_id {
            req = req.header("anthropic-organization-id", org_id);
        }

        let response = req
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Claude API")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            anyhow::bail!("Claude API error ({}): {}", status, body);
        }

        let claude_response: ClaudeResponse = response
            .json()
            .await
            .context("Failed to parse Claude API response")?;

        // Extract text from the first content block
        let text = claude_response
            .content
            .first()
            .map(|block| block.text.clone())
            .unwrap_or_else(|| "No response from Claude".to_string());

        Ok(text)
    }

    /// Check if the API key is valid by making a test request
    pub async fn verify_credentials(&self) -> Result<bool> {
        let messages = vec![Message::user("Hi")];

        match self.send_message(messages).await {
            Ok(_) => Ok(true),
            Err(e) => {
                log::warn!("Credential verification failed: {}", e);
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let user_msg = Message::user("Hello");
        assert_eq!(user_msg.role, Role::User);
        assert_eq!(user_msg.content, "Hello");

        let assistant_msg = Message::assistant("Hi there!");
        assert_eq!(assistant_msg.role, Role::Assistant);
        assert_eq!(assistant_msg.content, "Hi there!");
    }

    #[test]
    fn test_message_serialization() {
        let msg = Message::user("Test message");
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("user"));
        assert!(json.contains("Test message"));
    }
}
