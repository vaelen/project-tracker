# Configuration Documentation

## Overview

Project Tracker uses a TOML-based configuration file to manage application settings. The configuration file allows you to customize the application's behavior, set API credentials, configure data storage locations, and control logging output.

## Configuration File Location

### Default Location

```
~/.claude-tracker/config.toml
```

The configuration file is automatically created in this location on first run if it doesn't already exist.

### Custom Location

You can specify a custom configuration file location using the `--config` or `-c` command-line flag:

```bash
track --config /path/to/custom-config.toml projects list
```

This flag is global and works with all commands.

## Configuration File Format

The configuration file uses TOML (Tom's Obvious, Minimal Language) format. TOML is a simple, human-readable configuration file format.

### Example Configuration

```toml
# Project Tracker Configuration File

# Anthropic API Key
# Required for Claude AI integration
# Get your API key from: https://console.anthropic.com/
api_key = "sk-ant-api03-..."

# Data Storage Directory
# Directory where all application data (including SQLite database) is stored
# Supports tilde (~) expansion for home directory
data_dir = "~/.claude-tracker/data"

# Jira Configuration
# Base URL for Jira tickets (include trailing slash)
# Ticket numbers (e.g., PROJ-123) will be appended to this URL
jira_url = "https://jira.company.com/browse/"

# Default email domain for your organization
# When adding people, if only a name is provided, this domain will be suggested
default_email_domain = "company.com"

# Logging Configuration
[logging]
# Logging level: trace, debug, info, warn, error
level = "info"
```

## Configuration Options

### Top-Level Options

#### `api_key` (String, Required)

Your Anthropic API key for Claude integration.

**Type:** String
**Required:** Yes
**Default:** `"your-anthropic-api-key-here"`
**Example:** `"sk-ant-api03-..."`

**Description:** This key is used to authenticate with the Anthropic API for Claude AI features. You must replace the default placeholder value with your actual API key.

**How to Get:**
1. Visit https://console.anthropic.com/
2. Sign in or create an account
3. Navigate to API Keys section
4. Generate a new API key
5. Copy the key and paste it into your config file

**Security Note:** Keep your API key secure. Do not commit configuration files containing real API keys to version control.

---

#### `data_dir` (String, Required)

Path to the directory where all application data is stored.

**Type:** String
**Required:** Yes
**Default:** `"~/.claude-tracker/data"`
**Example:** `"/home/user/documents/claude-tracker-data"`

**Description:** All application data is stored in an SQLite database within this directory. The path supports tilde (`~`) expansion for the home directory.

**Files Created:**
- `claude-tracker.db` - SQLite database containing all application data

**Notes:**
- The directory is created automatically if it doesn't exist
- Use absolute paths or tilde expansion for reliability
- Ensure the application has read/write permissions to this location

---

#### `jira_url` (String, Optional)

Base URL for Jira ticket links.

**Type:** String
**Required:** No
**Default:** `"https://jira.company.com/browse/"`
**Example:** `"https://yourcompany.atlassian.net/browse/"`

**Description:** When you enter Jira ticket numbers in the application (like `PROJ-123` for initiatives or `PROJ-456` for epics), only the ticket number is stored in the database. The application automatically constructs full URLs by combining this base URL with the ticket number.

**How It Works:**
```
Stored ticket number: "PROJ-123"
jira_url setting: "https://jira.company.com/browse/"
Generated URL: "https://jira.company.com/browse/PROJ-123"
```

**Configuration Examples:**

Jira Cloud (Atlassian):
```toml
jira_url = "https://yourcompany.atlassian.net/browse/"
```

Self-Hosted Jira:
```toml
jira_url = "https://jira.company.com/browse/"
```

Custom Subdomain:
```toml
jira_url = "https://tickets.company.com/browse/"
```

**Notes:**
- Include the trailing slash in the URL
- This design allows you to change your Jira instance URL without updating all stored ticket references
- The URL is only used when displaying links, not when storing data

---

#### `default_email_domain` (String, Optional)

Default email domain for people in your organization.

**Type:** String
**Required:** No
**Default:** `"company.com"`
**Example:** `"yourcompany.com"`

**Description:** When adding new people to the system, the application uses this domain as the default for email addresses. This simplifies data entry since most people in your organization will have the same email domain.

**How It Works:**
- When prompted to add a new person, you can enter just their name
- The application suggests an email address using this domain
- For example, with `default_email_domain = "company.com"`, entering "Alice Smith" suggests `alice.smith@company.com`
- You can always override the suggestion and enter a different email address

**Configuration Examples:**

Corporate Domain:
```toml
default_email_domain = "company.com"
```

Organization Domain:
```toml
default_email_domain = "yourorg.org"
```

Multiple Domains:
```toml
# Set the most common one as default
default_email_domain = "company.com"
# You can still manually enter other domains (e.g., contractor@vendor.com)
```

**Notes:**
- This is only a default suggestion, not a restriction
- You can enter any email address regardless of this setting
- Email addresses are used as unique identifiers for people in the system

---

### Logging Section

The `[logging]` section controls application logging behavior.

#### `logging.level` (String, Optional)

The minimum log level to display.

**Type:** String
**Required:** No
**Default:** `"info"`
**Valid Values:** `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"`
**Example:** `"debug"`

**Description:** Controls the verbosity of log output. Lower levels include all higher levels.

**Log Levels (from most to least verbose):**
1. **trace** - Very detailed tracing information, including internal state
2. **debug** - Debugging information useful for troubleshooting
3. **info** - General informational messages (recommended for normal use)
4. **warn** - Warning messages for potentially problematic situations
5. **error** - Error messages for failures and critical issues

**Examples:**

```toml
# Minimal logging (errors only)
[logging]
level = "error"

# Verbose logging (for debugging)
[logging]
level = "debug"

# Default (recommended)
[logging]
level = "info"
```

## Environment Variables

Currently, Project Tracker does not use environment variables for configuration. All configuration is done through the TOML configuration file.

## Configuration Validation

The application validates the configuration file on startup and will report errors if:
- The file is not valid TOML format
- Required fields are missing
- Field types are incorrect
- File permissions prevent reading the configuration

## Creating and Managing Configuration

### First-Time Setup

1. Run any command to create the default configuration:
   ```bash
   track projects list
   ```

2. The application will create `~/.claude-tracker/config.toml` with default values

3. Edit the configuration file:
   ```bash
   # Linux/macOS
   nano ~/.claude-tracker/config.toml

   # Or use your preferred editor
   code ~/.claude-tracker/config.toml
   ```

4. Update the `api_key` with your actual Anthropic API key

5. Run the application again to verify the configuration is correct

### Multiple Configurations

You can maintain multiple configuration files for different environments or use cases:

```bash
# Personal projects
track --config ~/.claude-tracker/personal.toml projects list

# Work projects
track --config ~/.claude-tracker/work.toml projects list

# Testing
track --config /tmp/test-config.toml projects list
```

### Backup and Version Control

**DO:**
- Keep a backup of your configuration file
- Use version control for the configuration file structure (without API keys)
- Document any custom settings in your team

**DON'T:**
- Commit API keys to version control
- Share configuration files containing API keys
- Store API keys in plain text in shared locations

**Recommended Approach:**

1. Create a template configuration file (`config.toml.template`):
   ```toml
   api_key = "REPLACE_WITH_YOUR_API_KEY"
   data_dir = "~/.claude-tracker/data"

   [logging]
   level = "info"
   ```

2. Add the real config to `.gitignore`:
   ```
   ~/.claude-tracker/config.toml
   config.toml
   *.secret.toml
   ```

3. Users copy the template and add their own API key

## Configuration Examples

### Minimal Configuration

```toml
api_key = "sk-ant-api03-..."
data_dir = "~/.claude-tracker/data"
jira_url = "https://jira.company.com/browse/"
default_email_domain = "company.com"
```

### Development Configuration

```toml
api_key = "sk-ant-api03-..."
data_dir = "/tmp/claude-tracker-dev"
jira_url = "https://jira-dev.company.com/browse/"
default_email_domain = "company.com"

[logging]
level = "debug"
```

### Production Configuration

```toml
api_key = "sk-ant-api03-..."
data_dir = "/var/lib/claude-tracker/data"
jira_url = "https://jira.company.com/browse/"
default_email_domain = "company.com"

[logging]
level = "warn"
```

### Atlassian Jira Cloud

```toml
api_key = "sk-ant-api03-..."
data_dir = "~/.claude-tracker/data"
jira_url = "https://yourcompany.atlassian.net/browse/"
default_email_domain = "yourcompany.com"

[logging]
level = "info"
```

## Troubleshooting

### "Failed to read config file"

**Cause:** Configuration file doesn't exist or cannot be read.

**Solution:**
- Check that the file exists at the expected location
- Verify file permissions (must be readable by your user)
- Try deleting the file and letting the app recreate it

### "Failed to parse config file"

**Cause:** Invalid TOML syntax.

**Solution:**
- Check for syntax errors (missing quotes, brackets, etc.)
- Validate using an online TOML validator
- Compare with the example configurations above
- Recreate from the default template

### "Could not determine home directory"

**Cause:** The home directory environment variable is not set.

**Solution:**
- Use absolute paths instead of tilde expansion
- Set the `HOME` environment variable
- Use the `--config` flag with an absolute path

### API Key Not Working

**Cause:** Invalid or missing API key.

**Solution:**
- Verify the API key is correct (starts with `sk-ant-`)
- Ensure there are no extra spaces or newlines
- Check that you've saved the file after editing
- Verify the key is still valid in the Anthropic console

## Future Configuration Options

The following options are planned for future releases:

### Planned Options

```toml
# API Configuration (planned)
[api]
base_url = "https://api.anthropic.com"  # Custom API endpoint
timeout = 30  # Request timeout in seconds
retry_attempts = 3  # Number of retry attempts

# Database Configuration (planned)
[database]
type = "sqlite"  # Future: support for databases
path = "~/.claude-tracker/data.db"

# UI Configuration (planned)
[ui]
theme = "system"  # Options: light, dark, system
font_size = 12
date_format = "YYYY-MM-DD"

# Notifications (planned)
[notifications]
enabled = true
deadline_reminder_days = 7  # Days before deadline to notify

# Export Configuration (planned)
[export]
default_format = "markdown"  # Default export format
include_timestamps = true
```

**Note:** These options are not yet implemented and are subject to change.

## Version History

### Version 0.1.0 (Current)

Initial configuration system:
- `api_key` - Anthropic API key for Claude integration
- `data_dir` - Data storage directory (SQLite database location)
- `jira_url` - Base URL for Jira ticket links
- `default_email_domain` - Default email domain for organization
- `logging.level` - Log level configuration

## See Also

- [Storage Documentation](storage.md) - Data storage formats and structure
- [README.md](../README.md) - General application documentation
- [CLAUDE.md](../CLAUDE.md) - Development guidelines
