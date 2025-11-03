// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Authentication credentials for Claude API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// Session key for API authentication
    pub session_key: String,

    /// Organization ID (optional)
    pub organization_id: Option<String>,
}

impl Credentials {
    /// Get the default credentials file path (~/.claude-tracker/credentials.json)
    pub fn default_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("Could not determine home directory")?;
        Ok(home.join(".claude-tracker").join("credentials.json"))
    }

    /// Load credentials from the default location
    pub fn load() -> Result<Self> {
        let path = Self::default_path()?;
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read credentials file: {}", path.display()))?;

        let credentials: Credentials = serde_json::from_str(&contents)
            .context("Failed to parse credentials")?;

        Ok(credentials)
    }

    /// Save credentials to the default location
    pub fn save(&self) -> Result<()> {
        let path = Self::default_path()?;

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        let contents = serde_json::to_string_pretty(self)
            .context("Failed to serialize credentials")?;

        fs::write(&path, contents)
            .with_context(|| format!("Failed to write credentials file: {}", path.display()))?;

        log::info!("Credentials saved to: {}", path.display());
        Ok(())
    }

    /// Check if credentials exist
    pub fn exists() -> bool {
        Self::default_path()
            .map(|path| path.exists())
            .unwrap_or(false)
    }

    /// Delete credentials
    pub fn delete() -> Result<()> {
        let path = Self::default_path()?;
        if path.exists() {
            fs::remove_file(&path)
                .with_context(|| format!("Failed to delete credentials file: {}", path.display()))?;
            log::info!("Credentials deleted from: {}", path.display());
        }
        Ok(())
    }
}

/// Authentication status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStatus {
    pub authenticated: bool,
    pub organization_id: Option<String>,
}

/// Get authentication status
pub fn get_auth_status() -> AuthStatus {
    if Credentials::exists() {
        if let Ok(creds) = Credentials::load() {
            return AuthStatus {
                authenticated: true,
                organization_id: creds.organization_id,
            };
        }
    }

    AuthStatus {
        authenticated: false,
        organization_id: None,
    }
}

/// Authentication URL for initiating the auth flow
pub const AUTH_URL: &str = "https://claude.ai/login";

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_credentials_serialize() {
        let creds = Credentials {
            session_key: "test-key".to_string(),
            organization_id: Some("test-org".to_string()),
        };

        let json = serde_json::to_string(&creds).unwrap();
        assert!(json.contains("test-key"));
        assert!(json.contains("test-org"));
    }

    #[test]
    fn test_credentials_save_and_load() {
        let dir = tempdir().unwrap();
        let creds_path = dir.path().join("credentials.json");

        let creds = Credentials {
            session_key: "test-session-key".to_string(),
            organization_id: Some("test-org-id".to_string()),
        };

        // Save to custom path
        let json = serde_json::to_string_pretty(&creds).unwrap();
        fs::write(&creds_path, json).unwrap();

        // Load and verify
        let loaded_json = fs::read_to_string(&creds_path).unwrap();
        let loaded: Credentials = serde_json::from_str(&loaded_json).unwrap();

        assert_eq!(loaded.session_key, "test-session-key");
        assert_eq!(loaded.organization_id, Some("test-org-id".to_string()));
    }
}
