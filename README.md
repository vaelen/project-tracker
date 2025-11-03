# Project Tracker

An intelligent project and resource management application for software engineering managers, powered by Claude AI.

## Overview

Project Tracker helps engineering managers track projects, initiatives, deadlines, employee information, and stakeholder interactions. It features an embedded Claude Agent that provides intelligent assistance through natural language interactions.

## Features

- **Project Management**: Track current projects and their status
- **Initiative Tracking**: Monitor corporate initiatives and goals
- **Deadline Management**: Keep track of upcoming deadlines and milestones
- **Employee Information**: Manage team member details and leave schedules
- **Resource Allocation**: Plan and visualize which employees work on which projects
- **Stakeholder Management**: Track stakeholders and their interactions
- **Status Reports**: Generate comprehensive status reports
- **Note Taking**: Capture and organize notes with intelligent assistance
- **Daily Planning**: Plan and track daily resource allocation

## Interfaces

Project Tracker provides two ways to interact with your data:

- **Native GUI**: Tauri-based graphical interface (React frontend) for interactive use
- **CLI**: Full-featured command-line interface for automation and scripting

Both interfaces share the same core Rust library, ensuring consistent behavior and zero code duplication.

## Data Storage

All data is stored in human-readable text formats:
- **Markdown**: For notes, documentation, and reports
- **TSV**: For structured data (projects, employees, allocations)

This approach ensures your data is:
- Version control friendly
- Easy to backup and sync
- Human readable and editable
- Portable across systems

## Requirements

### For Users
- macOS or Linux operating system
- No additional dependencies (self-contained executable)

### For Developers
- Rust 1.70+ (with cargo)
- Node.js 18+ and npm (for frontend development)
- Git

#### Platform-Specific Prerequisites

**Linux (Ubuntu/Debian):**
```bash
sudo apt install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev
```

**Linux (Fedora/RHEL):**
```bash
sudo dnf install gtk3-devel webkit2gtk4.0-devel libappindicator-gtk3-devel librsvg2-devel
```

**Linux (Arch):**
```bash
sudo pacman -S gtk3 webkit2gtk libappindicator-gtk3 librsvg
```

**macOS:**
- Xcode Command Line Tools: `xcode-select --install`

## Installation

### From Release (Coming Soon)
Download the appropriate executable for your platform from the releases page and run it.

### From Source

**Important:** Before building from source, make sure to install the platform-specific prerequisites listed in the Requirements section above.

#### CLI Only
```bash
# Clone the repository
git clone <repository-url>
cd project-tracker

# Build the CLI
cargo build --release

# Run the CLI
./target/release/track --help
```

#### GUI Application
```bash
# Clone the repository
git clone <repository-url>
cd project-tracker

# Install platform prerequisites first (see Requirements section)
# For Ubuntu/Debian:
# sudo apt install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev

# Install frontend dependencies
cd ui && npm install && cd ..

# Development mode (with hot reload)
cd src-tauri && cargo tauri dev

# Or build for production
cd src-tauri && cargo tauri build
```

## Configuration

On first run, Project Tracker will create a configuration file at `~/.project-tracker/config.toml`. You need to edit this file and add your Anthropic API key.

### Configuration File Location

Default: `~/.project-tracker/config.toml`

You can override the configuration file location using the `--config` or `-c` flag:

```bash
track --config /path/to/config.toml projects list
```

### Configuration Options

```toml
# Anthropic API Key (required)
api_key = "your-anthropic-api-key-here"

# Data Storage Directory
data_dir = "~/.project-tracker/data"

# Logging Configuration
[logging]
level = "info"  # Options: trace, debug, info, warn, error
```

Get your Anthropic API key from: https://console.anthropic.com/

For detailed information about all configuration options, see [docs/config.md](docs/config.md).

### Data Storage

All data is stored in `~/.project-tracker/data/` by default (configurable via `data_dir` in config). The application will automatically create the following subdirectories:

- `projects/` - Project information
- `employees/` - Employee records
- `deadlines/` - Deadline tracking
- `initiatives/` - Corporate initiatives
- `stakeholders/` - Stakeholder information
- `allocations/` - Resource allocations
- `interactions/` - Stakeholder interactions
- `notes/` - Notes and documentation
- `reports/` - Generated reports

For detailed information about data formats and storage structure, see [docs/storage.md](docs/storage.md).

## Usage

### GUI Mode
```bash
# Launch the GUI
./track
```

### CLI Mode
```bash
# View help
track --help

# List projects
track projects list

# Add a new project
track projects add "New Feature Development"

# Generate status report
track report --format markdown

# Use custom config file
track --config /path/to/config.toml projects list
```

## Development

### Project Structure
```
project-tracker/
├── src/                    # Rust source (shared library + CLI)
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Shared library root
│   ├── agent/             # Claude API integration
│   ├── cli/               # CLI command handlers
│   ├── core/              # Core business logic
│   ├── storage/           # File I/O and data persistence
│   ├── tools/             # Claude agent tool definitions
│   └── utils/             # Utility functions
├── src-tauri/             # Tauri backend
│   ├── src/main.rs        # Tauri app entry
│   ├── Cargo.toml         # Tauri dependencies
│   └── tauri.conf.json    # Tauri configuration
├── ui/                    # React frontend
│   ├── src/               # React components
│   ├── package.json       # Frontend dependencies
│   └── vite.config.ts     # Vite configuration
├── tests/                 # Rust tests
├── docs/                  # Detailed documentation
├── data/                  # Default data directory
└── Cargo.toml             # Rust workspace configuration
```

### Building

#### CLI Binary
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# The binary will be at: target/release/track
```

#### GUI Application
```bash
# Install frontend dependencies
cd ui && npm install && cd ..

# Development mode (hot reload)
cd src-tauri && cargo tauri dev

# Production build
cd src-tauri && cargo tauri build

# Installers will be in: src-tauri/target/release/bundle/
```

### Testing

All functions are thoroughly tested with >90% code coverage requirement.

#### Rust Tests
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Generate coverage report (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

#### Frontend Tests
```bash
cd ui
npm test
npm run test:coverage
```

### Contributing

See [CLAUDE.md](CLAUDE.md) for detailed development guidelines and architectural decisions.

**Documentation:**
- [docs/config.md](docs/config.md) - Configuration file format and options
- [docs/storage.md](docs/storage.md) - Data storage formats and structure

When adding new configuration options, be sure to update [docs/config.md](docs/config.md).

### Troubleshooting

#### Build fails with "system library not found" errors

If you see errors like `The system library 'gdk-3.0' required by crate 'gdk-sys' was not found`, you need to install the platform-specific prerequisites. See the **Platform-Specific Prerequisites** section under Requirements.

**Linux:**
```bash
# Ubuntu/Debian
sudo apt install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev

# Fedora/RHEL
sudo dnf install gtk3-devel webkit2gtk4.0-devel libappindicator-gtk3-devel librsvg2-devel

# Arch
sudo pacman -S gtk3 webkit2gtk libappindicator-gtk3 librsvg
```

**macOS:**
```bash
xcode-select --install
```

#### Icon-related errors during build

If you see errors about missing or invalid icons, ensure that the `src-tauri/icons/` directory contains valid RGBA PNG images. You can regenerate them using ImageMagick:

```bash
cd src-tauri
for size in 32x32 128x128 256x256 512x512; do
  convert -size ${size%x*}x${size#*x} xc:none -background '#448AFF' -alpha set -channel RGBA -evaluate set 100% PNG32:icons/${size}.tmp.png
done
mv icons/32x32.tmp.png icons/32x32.png
mv icons/128x128.tmp.png icons/128x128.png
mv icons/256x256.tmp.png icons/128x128@2x.png
mv icons/512x512.tmp.png icons/icon.png
```

## License

MIT License - Copyright 2025 Andrew C. Young <andrew@vaelen.org>

See [LICENSE](LICENSE) for full license text.

## Support

For issues and feature requests, please use the project's issue tracker.
