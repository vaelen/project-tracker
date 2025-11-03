# Configuration Documentation

## Overview

Project Tracker uses a TOML-based configuration file to manage application settings. The configuration file allows you to customize the application's behavior, configure data storage locations, set Jira integration, and control logging output.

**Note:** For AI assistant integration via the MCP server, see the [MCP Server Configuration](#mcp-server-configuration) section below.

## Configuration File Location

### Default Location

```
~/.project-tracker/config.toml
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

# Data Storage Directory
# Directory where all application data (including SQLite database) is stored
# Supports tilde (~) expansion for home directory
data_dir = "~/.project-tracker"

# Jira Configuration
# Base URL for Jira tickets (include trailing slash)
# Ticket numbers (e.g., PROJ-123) will be appended to this URL
jira_url = "https://jira.company.com/browse/"

# Default email domain for your organization
# When adding people, if only a name is provided, this domain will be suggested
default_email_domain = "company.com"

# Available project types
# These are presented as options when creating/editing projects
project_types = ["Personal", "Team", "Company"]

# Logging Configuration
[logging]
# Logging level: trace, debug, info, warn, error
level = "info"
```

## Configuration Options

### Top-Level Options

#### `data_dir` (String, Required)

Path to the directory where all application data is stored.

**Type:** String
**Required:** Yes
**Default:** `"~/.project-tracker"`
**Example:** `"/home/user/documents/project-tracker"`

**Description:** All application data is stored in an SQLite database within this directory. The path supports tilde (`~`) expansion for the home directory.

**Files Created:**
- `project-tracker.db` - SQLite database containing all application data (projects, people, milestones, notes)

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

#### `project_types` (Array of Strings, Optional)

List of available project types.

**Type:** Array of Strings
**Required:** No
**Default:** `["Personal", "Team", "Company"]`
**Example:** `["Personal", "Team", "Department", "Company"]`

**Description:** Defines the project types that are available when creating or editing projects. These types help categorize projects by scope and ownership.

**How It Works:**
- When creating a project, you can select one of these types
- Types are stored in the database with each project
- You can add custom types to match your organization's structure

**Configuration Examples:**

Default Types:
```toml
project_types = ["Personal", "Team", "Company"]
```

Custom Organization Structure:
```toml
project_types = ["Personal", "Squad", "Tribe", "Organization"]
```

Engineering Team Structure:
```toml
project_types = ["Personal", "Backend", "Frontend", "Infrastructure", "Cross-Team"]
```

**Notes:**
- Order matters - types are presented in the order listed
- Existing projects with types not in this list will still display correctly
- Changing this list does not affect existing project data
- Type names should be concise (1-2 words recommended)

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

2. The application will create `~/.project-tracker/config.toml` with default values

3. Edit the configuration file:
   ```bash
   # Linux/macOS
   nano ~/.project-tracker/config.toml

   # Or use your preferred editor
   code ~/.project-tracker/config.toml
   ```

4. Optionally customize settings (Jira URL, email domain, project types, etc.)

5. Run the application again to verify the configuration is correct

### Multiple Configurations

You can maintain multiple configuration files for different environments or use cases:

```bash
# Personal projects
track --config ~/.project-tracker/personal.toml projects list

# Work projects
track --config ~/.project-tracker/work.toml projects list

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
   data_dir = "~/.project-tracker"
   jira_url = "https://jira.company.com/browse/"
   default_email_domain = "company.com"
   project_types = ["Personal", "Team", "Company"]

   [logging]
   level = "info"
   ```

2. Add the real config and data to `.gitignore`:
   ```
   ~/.project-tracker/
   config.toml
   *.secret.toml
   ```

3. Users copy the template and customize for their environment

## Configuration Examples

### Minimal Configuration

```toml
data_dir = "~/.project-tracker"
jira_url = "https://jira.company.com/browse/"
default_email_domain = "company.com"
project_types = ["Personal", "Team", "Company"]
```

### Development Configuration

```toml
data_dir = "/tmp/project-tracker-dev"
jira_url = "https://jira-dev.company.com/browse/"
default_email_domain = "company.com"
project_types = ["Personal", "Team", "Company"]

[logging]
level = "debug"
```

### Production Configuration

```toml
data_dir = "/var/lib/project-tracker"
jira_url = "https://jira.company.com/browse/"
default_email_domain = "company.com"
project_types = ["Personal", "Team", "Company"]

[logging]
level = "warn"
```

### Atlassian Jira Cloud

```toml
data_dir = "~/.project-tracker"
jira_url = "https://yourcompany.atlassian.net/browse/"
default_email_domain = "yourcompany.com"
project_types = ["Personal", "Team", "Company"]

[logging]
level = "info"
```

## MCP Server Configuration

Project Tracker includes a Model Context Protocol (MCP) server that allows AI assistants like Claude Desktop to access your project data.

### MCP Server Setup

The MCP server is configured separately from the Project Tracker application itself. The server uses the same database and configuration as the CLI and GUI applications.

#### Building the MCP Server

```bash
# Build the MCP server binary
cargo build --release --bin track-mcp

# The binary will be at: target/release/track-mcp
```

#### Configuring Claude Desktop

To use the MCP server with Claude Desktop, add the following to your Claude Desktop configuration file:

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`

**Linux:** `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "project-tracker": {
      "command": "/absolute/path/to/project-tracker/target/release/track-mcp",
      "args": []
    }
  }
}
```

**Important:** Replace `/absolute/path/to/project-tracker` with the actual absolute path to your project directory.

#### MCP Server Configuration Options

The MCP server uses the same configuration file as the CLI and GUI (`~/.project-tracker/config.toml`). It specifically uses:

- **`data_dir`** - Location of the SQLite database
- **`jira_url`** - For generating Jira ticket links in responses
- **`default_email_domain`** - For validating email addresses
- **`project_types`** - For project type validation

#### MCP Server Environment

The MCP server:
- Reads configuration from `~/.project-tracker/config.toml`
- Accesses the same SQLite database as CLI and GUI
- Logs to stderr (stdout is reserved for MCP protocol)
- Runs as a separate process managed by Claude Desktop

#### Testing MCP Server

To verify the MCP server is working:

1. Build and configure the server as described above
2. Restart Claude Desktop
3. Look for "project-tracker" in the Claude Desktop MCP servers list
4. Try a test prompt: "Show me all my projects"

Claude should be able to list your projects, create new ones, manage people, and interact with all Project Tracker data.

#### Troubleshooting MCP Server

**Server not appearing in Claude Desktop:**
- Verify the path to `track-mcp` binary is absolute and correct
- Check that the binary has execute permissions: `chmod +x target/release/track-mcp`
- Restart Claude Desktop after configuration changes
- Check Claude Desktop logs for error messages

**Server starts but can't access data:**
- Verify `~/.project-tracker/config.toml` exists and is valid
- Check that `data_dir` in the config points to the correct location
- Ensure the database file exists: `~/.project-tracker/project-tracker.db`
- Verify file permissions allow the MCP server to read/write the database

**Database locked errors:**
- SQLite databases can only have one writer at a time
- Close the GUI application if it's running
- Ensure no other processes are accessing the database

For more information about available MCP tools, see the main [README.md](../README.md#mcp-server-mode).

---

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

### Database Access Errors

**Cause:** Database file cannot be accessed or is locked.

**Solution:**
- Verify the database file exists at `<data_dir>/project-tracker.db`
- Check file permissions (must be readable and writable)
- Ensure only one application instance is writing to the database
- Close the GUI if using the CLI or MCP server simultaneously

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
path = "~/.project-tracker/data.db"

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
- `data_dir` - Data storage directory (SQLite database location)
- `jira_url` - Base URL for Jira ticket links
- `default_email_domain` - Default email domain for organization
- `project_types` - Available project types
- `logging.level` - Log level configuration
- MCP server support for Claude Desktop integration

## See Also

- [Storage Documentation](storage.md) - Data storage formats and structure
- [README.md](../README.md) - General application documentation
- [CLAUDE.md](../CLAUDE.md) - Development guidelines
