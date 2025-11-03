# Project Tracker - Development Documentation

## Project Overview

Project Tracker is a project and resource management application designed for software engineering managers at large tech companies. It integrates a Claude Agent using the Claude Agent SDK to provide intelligent assistance for tracking projects, initiatives, deadlines, employee information, and stakeholder interactions.

## Core Requirements

### Technology Stack
- **Language**: Rust
- **GUI Framework**: Tauri (React/TypeScript frontend, Rust backend)
- **Agent Integration**: anthropic-sdk-rust for Claude API
- **Data Storage**: Text-based formats (Markdown, TSV)
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

### Claude Agent Integration
- The embedded Claude agent interacts with custom tools
- Tools provided by the tracker application for CRUD operations
- Agent manages data files in text formats (Markdown/TSV)

### Dual Interface
1. **GUI**: Native application interface
2. **CLI**: Command-line interface with full feature parity

### Data Storage
- All data stored in text-based formats
- Markdown for documentation and notes
- TSV for structured data (employees, projects, allocations)
- Human-readable and version control friendly

## Development Guidelines

### Code Structure
```
project-tracker/
├── src/                    # Rust source code
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Library root
│   ├── agent/             # Claude API integration
│   ├── cli/               # CLI command handlers
│   ├── core/              # Core business logic
│   │   ├── projects.rs    # Project management
│   │   ├── employees.rs   # Employee tracking
│   │   ├── deadlines.rs   # Deadline management
│   │   └── reports.rs     # Report generation
│   ├── storage/           # File I/O and data persistence
│   ├── tools/             # Claude agent tool definitions
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
- Integration tests for Claude agent tools
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

3. **Agent Tool Development**
   - Define tool schema
   - Implement tool handler
   - Test tool integration
   - Document tool usage

## Key Design Decisions

### Why Text-Based Storage?
- Version control friendly
- Human readable and editable
- Easy backup and sync
- No database dependencies

### Why Native Executable?
- Better user experience (no runtime installation)
- Easier distribution
- More professional deployment

### Why Dual Interface?
- GUI for daily interactive use
- CLI for automation and scripting
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

### Claude Agent Integration
- Uses `anthropic-sdk-rust` crate (or `reqwest` for direct API calls)
- Agent tools implemented as Rust functions
- Tools can read/write data files (Markdown, TSV)
- Same agent integration used by both CLI and GUI

### Data Flow

**CLI Mode:**
```
User → CLI Args → clap Parser → Core Functions → Storage Layer → Files
```

**GUI Mode:**
```
User → React UI → Tauri IPC → Tauri Commands → Core Functions → Storage Layer → Files
                    ↓
            Claude Agent (backend)
```

### Key Dependencies
- **anthropic-sdk-rust** or **reqwest**: Claude API communication
- **clap**: CLI argument parsing
- **tauri**: GUI framework
- **serde**: Serialization/deserialization
- **tokio**: Async runtime
- **anyhow**: Error handling
- **csv**: TSV file handling
- **pulldown-cmark** or **comrak**: Markdown parsing

## Next Steps

1. Initialize Cargo workspace with CLI and Tauri projects
2. Set up core library structure
3. Implement data models (structs for Projects, Employees, etc.)
4. Create storage layer for file I/O
5. Implement Claude agent integration
6. Build CLI commands
7. Create Tauri commands and React UI
8. Package for distribution

## Notes for Claude

- Prioritize code quality and test coverage
- Keep documentation synchronized with code
- Follow Rust best practices (ownership, borrowing, error handling)
- Use idiomatic Rust patterns
- Ensure all features work in both CLI and GUI
- Consider user experience in both interfaces
- Plan for extensibility (new data types, new tools)
- Leverage Rust's type system for correctness
- Use async/await for Claude API calls
