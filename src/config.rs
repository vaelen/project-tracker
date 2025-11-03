// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Anthropic API key
    pub api_key: String,

    /// Data storage directory
    pub data_dir: String,

    /// Jira base URL (e.g., "https://jira.company.com/browse/")
    #[serde(default = "default_jira_url")]
    pub jira_url: String,

    /// Default email domain (e.g., "company.com")
    #[serde(default = "default_email_domain")]
    pub default_email_domain: String,

    /// Available project types
    #[serde(default = "default_project_types")]
    pub project_types: Vec<String>,

    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,
}

fn default_jira_url() -> String {
    "https://jira.company.com/browse/".to_string()
}

fn default_email_domain() -> String {
    "company.com".to_string()
}

fn default_project_types() -> Vec<String> {
    vec![
        "Personal".to_string(),
        "Team".to_string(),
        "Company".to_string(),
    ]
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
        }
    }
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Config {
    /// Load configuration from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Get the default config file path (~/.project-tracker/config.toml)
    pub fn default_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("Could not determine home directory")?;

        Ok(home.join(".project-tracker").join("config.toml"))
    }

    /// Load configuration from the default location or create a default config
    pub fn load_or_default() -> Result<Self> {
        let default_path = Self::default_path()?;

        if default_path.exists() {
            Self::load(&default_path)
        } else {
            // Create default config directory and file
            let config_dir = default_path.parent().unwrap();
            fs::create_dir_all(config_dir)
                .with_context(|| format!("Failed to create config directory: {}", config_dir.display()))?;

            let default_config = Self::default();
            default_config.save(&default_path)?;

            log::info!("Created default configuration at: {}", default_path.display());
            log::warn!("Please edit the config file and add your Anthropic API key");

            Ok(default_config)
        }
    }

    /// Save configuration to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(path, contents)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Expand tilde (~) in path to home directory
    pub fn expand_path(&self, path: &str) -> Result<PathBuf> {
        if path.starts_with('~') {
            let home = dirs::home_dir()
                .context("Could not determine home directory")?;
            let path_without_tilde = path.strip_prefix("~/").unwrap_or(&path[1..]);
            Ok(home.join(path_without_tilde))
        } else {
            Ok(PathBuf::from(path))
        }
    }

    /// Get the expanded data directory path
    pub fn data_dir_path(&self) -> Result<PathBuf> {
        self.expand_path(&self.data_dir)
    }

    /// Ensure data directory exists
    pub fn ensure_data_dirs(&self) -> Result<()> {
        let data_dir = self.data_dir_path()?;
        fs::create_dir_all(&data_dir)
            .with_context(|| format!("Failed to create data directory: {}", data_dir.display()))?;
        log::debug!("Data directory initialized at: {}", data_dir.display());
        Ok(())
    }

    /// Get the full Jira ticket URL for a ticket number
    pub fn jira_ticket_url(&self, ticket: &str) -> String {
        format!("{}{}", self.jira_url, ticket)
    }

    /// Get the database file path
    pub fn database_path(&self) -> Result<PathBuf> {
        let data_dir = self.data_dir_path()?;
        Ok(data_dir.join("project-tracker.db"))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: "your-anthropic-api-key-here".to_string(),
            data_dir: "~/.project-tracker/data".to_string(),
            jira_url: default_jira_url(),
            default_email_domain: default_email_domain(),
            project_types: default_project_types(),
            logging: LoggingConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.api_key, "your-anthropic-api-key-here");
        assert_eq!(config.data_dir, "~/.project-tracker/data");
        assert_eq!(config.logging.level, "info");
    }

    #[test]
    fn test_config_serialize() {
        let config = Config::default();
        let toml = toml::to_string(&config).unwrap();
        assert!(toml.contains("api_key"));
        assert!(toml.contains("data_dir"));
    }

    #[test]
    fn test_config_save_and_load() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.toml");

        let config = Config::default();
        config.save(&config_path).unwrap();

        let loaded = Config::load(&config_path).unwrap();
        assert_eq!(loaded.api_key, config.api_key);
        assert_eq!(loaded.data_dir, config.data_dir);
    }

    #[test]
    fn test_expand_path() {
        let config = Config::default();

        // Test tilde expansion
        let expanded = config.expand_path("~/test/path").unwrap();
        assert!(!expanded.to_string_lossy().contains('~'));

        // Test absolute path
        let expanded = config.expand_path("/absolute/path").unwrap();
        assert_eq!(expanded, PathBuf::from("/absolute/path"));
    }

    #[test]
    fn test_ensure_data_dirs() {
        let dir = tempdir().unwrap();
        let mut config = Config::default();
        config.data_dir = dir.path().to_string_lossy().to_string();

        config.ensure_data_dirs().unwrap();

        // Check that subdirectories were created
        assert!(dir.path().join("projects").exists());
        assert!(dir.path().join("employees").exists());
        assert!(dir.path().join("deadlines").exists());
        assert!(dir.path().join("notes").exists());
        assert!(dir.path().join("reports").exists());
    }
}
