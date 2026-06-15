# Technology Stack: DigitalNZ Integration Hub (Rust Core)

## 1. Core Stack

### Programming Language & Runtime
- **Rust (Edition 2021):** Main development language for compile-time safety and peak execution performance.
- **Tokio:** Asynchronous runtime driving the non-blocking MCP server, CLI operations, and API integrations.

### Base Libraries & API Integration
- **HTTP Client:** `reqwest` (asynchronous, driven by `tokio`).
- **Serialization & Parsers:** `serde` & `serde_json` (compile-time, high-performance JSON serialization/deserialization).
- **Error Handling:** `thiserror` (for domain-specific errors) and `anyhow` (for application-level context).
- **Observability & Diagnostics:** `tracing` and `tracing-subscriber` (zero-overhead, async-aware logging routing diagnostics to `stderr`).

### User Interfaces & Applications
- **Command Line Interface (CLI):** `clap` (using the modern `derive` feature for strongly-typed arguments and auto-generated help).
- **MCP Server:** Async MCP implementation using standard JSON-RPC over stdin/stdout.

## 2. Infrastructure & Quality Control
- **Dependency & Package Management:** `pixi` (environment declaration, native compilation, and system-level cross-platform dependency resolution).
- **Linting & Formatting:** `cargo clippy` (configured with strict safety rules) and `cargo fmt`.
- **Testing & Mocking Frameworks:**
  - Standard test suite using `cargo test`.
  - `wiremock` for local, hermetic, and offline HTTP API mocking.
  - `proptest` for property-based generation.
- **CI/CD:** GitHub Actions using `swatinem/rust-cache` (to cache Cargo artifacts) and `cargo-dist` for cross-platform binary builds.
