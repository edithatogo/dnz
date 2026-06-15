# Track Specification: Scaffold CLI & MCP Skeleton (Rust Core)

## Overview
This track sets up the initial project workspace in Rust. It establishes the Cargo workspace structure, configures static analysis tooling (Clippy and Rustfmt), integrates `pixi` for dependencies, and creates native entry points for both the Command Line Interface (CLI) and the Model Context Protocol (MCP) server modules, including standard `tracing` loggers.

## User Stories / Requirements
- As a developer, I want a standard Cargo and Pixi workspace setup to ensure reproducible builds.
- As a developer, I want a compiled CLI tool (`dnz`) using `clap` that supports standard flags (`--help`, `-V`) to verify correct binary compilation.
- As a developer, I want a running skeleton of the async MCP server that initializes and responds via standard I/O streams, with errors outputting to standard error via `tracing`.
- As a CI/CD process, I want automated test execution that verifies lint rules, compilation, and basic tests using build caches.

## Technical Constraints
- **Language/Compiler:** Rust (Edition 2021).
- **Asynchronous Engine:** `tokio` runtime.
- **CLI Framework:** `clap` (derive interface).
- **Dependencies & Environments:** Declared in `Cargo.toml` and managed via `pixi`.
- **Diagnostics:** Structured logs via the `tracing` subscriber targeted to `stderr`.
- **Testing:** Standard test suite using `cargo test`.
