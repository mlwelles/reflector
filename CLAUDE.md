# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Reflector is a Rust CLI tool that mirrors satellite data from government sources (FTP and HTTP) to local storage. The application predicts and mirrors upstream data, maintaining a local cache of recent images/movies based on configurable loop periods.

## Common Commands

### Build and Development
- `cargo build` - Build the project
- `cargo run` - Run the main binary
- `make build` - Alternative build command via Makefile
- `make install` - Install the binary using `cargo install --path .`

### Testing
- `cargo test` - Run all tests
- `cargo test --no-run` - Compile tests without running them
- `cargo test -- --nocapture` - Run tests with output visible
- `make test` - Alternative test command via Makefile

### Code Quality
- `cargo fmt --all -- --check` - Check code formatting
- `cargo clippy -- -D warnings` - Run linting with warnings as errors
- `cargo fmt` - Format code

## Architecture

### Core Components

**Config System** (`src/config.rs`)
- `SourceConfig`: Defines mirror sources with remote/local paths, periods, and pathmakers
- Built-in presets for SDO (Solar Data Observatory) and GOES satellite data
- TOML-based configuration with sources array

**Mirror System** (`src/mirror.rs`)
- `Mirror`: Core abstraction that manages data mirroring from remote sources
- `MirrorStatus`: Tracks completeness (Empty, Partial, Full, Unimplemented)
- Handles time-based data fetching with configurable periods and loop periods

**Storage** (`src/store/`)
- `FileStore`: Local file system storage management
- `FileList`: Manages collections of stored files
- Supports flattened directory structures

**Remote Clients** (`src/remote/`)
- `RemoteClient`: Trait for different protocols (HTTP, FTP, Mock)
- Protocol-specific implementations in separate modules
- Supports both FTP and HTTP data sources

**Path Management** (`src/pathmaker/`)
- `PathMaker`: Converts between timestamps and remote file paths
- Time-based filename generation for different satellite data sources
- Handles different naming conventions (SDO, GOES formats)

**Time Handling** (`src/time_*.rs`)
- `TimeRange`: Time interval management
- `TimeList`: Collections of time-based data points
- Utilities for time-based data organization

### Data Flow

1. Config defines sources with periods and pathmakers
2. Mirror uses PathMaker to generate URLs for time ranges
3. RemoteClient fetches data from upstream sources
4. FileStore manages local caching with optional flattening
5. Capture represents individual mirrored files with metadata

## Testing

Tests are located in the `tests/` directory and use standard Rust testing patterns. The project uses `httpmock` for HTTP testing and `assert_cmd` for CLI testing.

## Configuration

The application uses TOML configuration files with `[[sources]]` arrays. Each source requires:
- `name`: Human-readable name
- `abbrev`: Short identifier
- `remote`: Base URL for data source
- `local`: Local storage path
- `pathmaker`: Path generation strategy
- `period`: Interval between captures (seconds)
- Optional: `offset`, `loop_period`, `flatten`