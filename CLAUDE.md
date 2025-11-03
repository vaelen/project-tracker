# Project Tracker - Development Documentation

## Project Overview

Project Tracker is a project and resource management application designed for software engineering managers at large tech companies. It provides AI assistance through a Model Context Protocol (MCP) server that allows Claude Desktop and other AI assistants to interact with your project data for tracking projects, initiatives, deadlines, employee information, and stakeholder interactions.

## Core Requirements

### Technology Stack
- **Language**: Rust
- **GUI Framework**: Tauri (React/TypeScript frontend, Rust backend)
- **AI Integration**: MCP server using rmcp SDK for Model Context Protocol
- **Data Storage**: SQLite database for structured data
- **CLI Framework**: clap (Rust command-line parser)
- **Packaging**: Native executable (single binary, no runtime dependencies)
- **Frontend**: React + TypeScript for Tauri webview
- **Build System**: Cargo (Rust) + npm/vite (frontend)

### License & Copyright
- **License**: MIT License
- **Copyright**: Copyright 2025 Andrew C. Young <andrew@vaelen.org>
- **Requirement**: All source code files must contain copyright header

### Quality Standards
- **Test Coverage**: Minimum 90% coverage required
- **Testing**: All functions must be thoroughly tested
- **Documentation**: Keep README.md and docs/ folder current

## Application Features

### Data Management
The application tracks:
1. **Projects**: Current projects and their status
2. **Corporate Initiatives**: Organization-wide initiatives
3. **Deadlines**: Upcoming deadlines and milestones
4. **Employee Information**: Team member details and leave schedules
5. **Resource Allocation**: Which employees work on which projects
6. **Stakeholders**: Project stakeholders and their roles
7. **Interactions**: Notes and history of stakeholder communications

### Core Capabilities
- Generate status reports
- Take and organize notes
- Plan resource allocation
- Daily planning and tracking

## Architecture

### MCP Server Integration
- Standalone MCP server binary exposes all application functionality
- Tools provided via Model Context Protocol for CRUD operations
- Works with Claude Desktop and other MCP-compatible AI assistants
- Shares the same SQLite database as CLI and GUI

### Multiple Interfaces
1. **GUI**: Native application interface
2. **CLI**: Command-line interface with full feature parity
3. **MCP Server**: AI assistant integration via Model Context Protocol

### Data Storage
- SQLite database for all structured data (projects, people, milestones, notes)
- Single database file at `~/.project-tracker/data/tracker.db`
- Markdown rendering for notes display in the UI
- Database schema versioning with migrations

## Development Guidelines

### Code Structure
```
project-tracker/
├── src/                    # Rust source code
│   ├── main.rs            # CLI entry point
│   ├── mcp_server.rs      # MCP server entry point
│   ├── lib.rs             # Library root
│   ├── cli/               # CLI command handlers
│   ├── core/              # Core business logic
│   │   ├── projects.rs    # Project management
│   │   ├── employees.rs   # Employee tracking
│   │   ├── deadlines.rs   # Deadline management
│   │   └── reports.rs     # Report generation
│   ├── db/                # Database models and repositories
│   ├── storage/           # File I/O and data persistence
│   └── utils/             # Utility functions
├── src-tauri/             # Tauri backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Tauri app entry point
│   │   ├── commands.rs    # Tauri IPC commands
│   │   └── menu.rs        # App menu definitions
│   ├── Cargo.toml         # Tauri dependencies
│   └── tauri.conf.json    # Tauri configuration
├── ui/                    # Frontend (React + TypeScript)
│   ├── src/
│   │   ├── App.tsx        # Main React app
│   │   ├── components/    # React components
│   │   ├── hooks/         # Custom React hooks
│   │   └── services/      # Tauri invoke wrappers
│   ├── package.json       # Frontend dependencies
│   └── vite.config.ts     # Vite configuration
├── tests/                 # Rust tests
├── docs/                  # Detailed documentation
├── data/                  # Default data directory
└── Cargo.toml             # Main Rust project manifest
```

### Copyright Header Templates

**Rust:**
```rust
// Copyright 2025 Andrew C. Young <andrew@vaelen.org>
//
// SPDX-License-Identifier: MIT
```

**TypeScript/JavaScript:**
```typescript
/**
 * Copyright 2025 Andrew C. Young <andrew@vaelen.org>
 *
 * SPDX-License-Identifier: MIT
 */
```

### Testing Requirements
- **Rust**: Unit and integration tests using built-in test framework
- **Frontend**: Jest + React Testing Library for UI components
- Integration tests for MCP server tools
- E2E tests for CLI commands
- GUI E2E tests using Tauri's testing utilities
- Maintain >90% code coverage (use cargo-tarpaulin for Rust)

### Build & Packaging
- **CLI**: Single Rust binary via `cargo build --release`
- **GUI**: Tauri bundles native executable with embedded webview
- Target platforms: macOS (Intel & Apple Silicon) and Linux (x86_64)
- No runtime dependencies (statically linked Rust binary)
- Frontend assets bundled into the binary
- Use `cargo-bundle` or Tauri's built-in bundler for distribution packages

## Development Workflow

1. **Feature Development**
   - Write tests first (TDD approach recommended)
   - Implement feature
   - Ensure tests pass and coverage meets requirements
   - Update documentation

2. **Documentation Updates**
   - Keep README.md current with build and usage instructions
   - Update this CLAUDE.md file with architectural decisions
   - Maintain detailed docs in docs/ folder

3. **MCP Tool Development**
   - Define tool using `#[tool]` attribute
   - Implement typed request/response structs with JsonSchema
   - Test tool via MCP protocol
   - Document tool in README.md

## Key Design Decisions

### Why SQLite?
- Single file database - easy to backup and sync
- No server process required - embedded database
- ACID transactions for data integrity
- Full-text search capabilities
- Mature and battle-tested

### Why Native Executable?
- Better user experience (no runtime installation)
- Easier distribution
- More professional deployment

### Why Multiple Interfaces?
- GUI for daily interactive use
- CLI for automation and scripting
- MCP server for AI assistant integration
- Flexibility for different workflows

## Architecture Details

### Shared Core Library
The core business logic is implemented as a Rust library (`src/lib.rs`) that is used by both:
1. **CLI binary** (`src/main.rs`) - Direct function calls
2. **Tauri backend** (`src-tauri/`) - Wrapped in Tauri commands for IPC

This ensures:
- Zero code duplication between CLI and GUI
- Consistent behavior across interfaces
- Shared testing coverage
- Single source of truth for business logic

### MCP Server Integration
- Uses `rmcp` crate (official Rust SDK for Model Context Protocol)
- Tools implemented as async Rust methods with `#[tool]` attribute
- Tools read/write to shared SQLite database
- MCP server runs as separate process, CLI and GUI are native binaries

### Data Flow

**CLI Mode:**
```
User → CLI Args → clap Parser → Core Functions → Database Layer → SQLite
```

**GUI Mode:**
```
User → React UI → Tauri IPC → Tauri Commands → Core Functions → Database Layer → SQLite
```

**MCP Mode:**
```
Claude Desktop → MCP Protocol → MCP Server → Core Functions → Database Layer → SQLite
```

### Key Dependencies
- **rmcp**: Model Context Protocol server implementation
- **clap**: CLI argument parsing
- **tauri**: GUI framework
- **serde**: Serialization/deserialization
- **schemars**: JSON schema generation for MCP tools
- **tokio**: Async runtime
- **anyhow**: Error handling
- **rusqlite**: SQLite database access

## Next Steps

1. ✅ Initialize Cargo workspace with CLI, MCP server, and Tauri projects
2. ✅ Set up core library structure
3. ✅ Implement data models (structs for Projects, People, Milestones, etc.)
4. ✅ Create database layer with SQLite
5. ✅ Implement MCP server integration
6. Build CLI commands
7. Create Tauri commands and React UI
8. Package for distribution

## Notes for Claude

- Prioritize code quality and test coverage
- Keep documentation synchronized with code
- Follow Rust best practices (ownership, borrowing, error handling)
- Use idiomatic Rust patterns
- Ensure all features work across CLI, GUI, and MCP server
- Consider user experience in all interfaces
- Plan for extensibility (new data types, new MCP tools)
- Leverage Rust's type system for correctness
- Use async/await for database and MCP operations
- MCP tools should use typed request structs with `schemars::JsonSchema` derive
