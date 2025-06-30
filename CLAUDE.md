# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is an AtCoder competitive programming environment supporting Go, Rust, Zig, and C++ languages. The project uses `atcoder-cli` (acc) and `Taskfile` to streamline contest preparation, testing, and submission workflows.

## Core Commands

### Contest Management
- `task new -- <contest_id>` - Create new contest directory with problem templates (run from repo root)
  - Creates `ABC/<contest_id>/` with subdirectories for each problem (a, b, c, etc.)
  - Each problem contains `go/`, `rust/`, `zig/`, and `cpp/` subdirectories with language-specific setups

### Development Workflow (run from language-specific directories)
- `task test` - Format, lint, and test solution against sample cases
- `task submit` - Submit solution to AtCoder
- `task fmt` - Format code (gofmt, cargo fmt, or zig fmt)
- `task lint` - Lint code (go vet, cargo clippy, or zig build-exe)

### Language-Specific Commands
- **Go**: `go run main.go`, `go mod init main`
- **Rust**: `cargo run`, `cargo new`, `cargo add <crate>`
- **Zig**: `zig run main.zig`, `zig build-exe main.zig`
- **C++**: `g++ -std=c++17 -o main main.cpp && ./main`

## Project Structure

```
at.coder/
├── Taskfile.yml              # Task definitions for development workflow
├── ABC/                      # Contest directories (auto-generated)
│   └── <contest_id>/
│       └── <problem>/
│           ├── tests/        # Sample test cases
│           ├── go/           # Go solution directory
│           ├── rust/         # Rust solution directory
│           └── zig/          # Zig solution directory
├── templates/default/        # Language templates for new problems
│   ├── go/
│   ├── rust/
│   ├── zig/
│   └── cpp/
└── learning/                 # Educational materials for all four languages
    ├── go/
    ├── rust/
    ├── zig/
    └── cpp/
```

## Language Setup Details

### Go Projects
- Uses `main.go` as entry point
- Automatically initializes Go module with `go mod init main`
- Submit file: `main.go`

### Rust Projects
- Uses standard Cargo project structure (`src/main.rs`)
- Automatically adds common competitive programming crates:
  - `proconio@0.3.6` for input parsing
  - `itertools@0.9.0` for iterator utilities
- Setup script `setup_rust.sh` creates Cargo project and adds dependencies
- Submit file: `src/main.rs`

### Zig Projects
- Uses `main.zig` as entry point
- Direct compilation and execution without package manager
- Submit file: `main.zig`

### C++ Projects
- Uses `main.cpp` as entry point
- Standard compilation with g++ and C++17 standard
- Submit file: `main.cpp`

## Testing Framework

Uses `online-judge-tools` (oj) for automated testing:
- Test files stored in `../tests/` relative to solution directory
- Supports all three languages with appropriate run commands
- Automatically downloads sample cases when creating new contests

## Development Best Practices

### Working Directory Convention
- **Contest creation**: Run from repository root
- **Problem solving**: Work within specific language directory (e.g., `ABC/abc123/a/rust/`)
- **Testing/submission**: Execute commands from the language directory you're working in

### Typical Workflow
1. `task new -- abc123` (from repo root)
2. `cd ABC/abc123/a/rust` (or preferred language: go, rust, zig, cpp)
3. Edit solution in `src/main.rs` (or `main.go`, `main.zig`, `main.cpp`)
4. `task test` to verify against samples
5. `task submit` to submit to AtCoder

### Code Organization
- Each language directory is self-contained
- Templates provide consistent starting points
- Learning materials available in `learning/` for language reference

## External Dependencies

Required tools:
- Go, Rust, Zig, C++ compilers (g++)
- Node.js (for atcoder-cli)
- Python (for online-judge-tools)
- Taskfile
- atcoder-cli (`acc`)
- online-judge-tools (`oj`)

The repository expects these tools to be properly installed and configured for AtCoder authentication.