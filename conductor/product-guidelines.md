# Product Guidelines: DigitalNZ Integration Hub (Rust Core)

## 1. Core Principles & Design Standards

### Compile-Time Safety & Schema-First Design
- Expose all tool schemas, configurations, and API payload definitions via compile-time validated structures (using Rust's type system and `serde`).
- Input parameter structures for CLI and MCP endpoints must be strictly typed, self-documenting, and descriptive to guide LLM interface mapping.

### Interface & Output Formats
- **CLI Output:** Defaults to clean, structured markdown tables or standard output text. Fully supports `--json` or `--format json` output flags utilizing stream-oriented JSON serialization for piping.
- **MCP Response:** Outputs clean, structured JSON or Markdown text. Maximizes compatibility with LLM context windows by offering field-filtered response payloads.
- **Serialization boundaries:** Strictly enforce zero-copy parsing where applicable (e.g. `serde` serialization).

### Network Resilience & Error Architecture
- Implement robust, non-blocking retry mechanisms with exponential backoff and jitter for the DigitalNZ API.
- Native panic messages must be caught or handled gracefully. Error messages returned to the CLI or MCP console must be structured, clear, and actionable.

## 2. Technical Quality & Automation

### Bleeding-Edge Standards
- Leverage modern Rust conventions (Edition 2021, async/await runtime, robust error propagation with `thiserror`/`anyhow`).
- Expose standardized Model Context Protocol (MCP) server capabilities.

### Automation & CI/CD
- **Strict Linting & Quality:** Enforce zero-warnings on `cargo clippy` and require exact formatting with `cargo fmt`.
- **Automated Testing:** Run comprehensive unit, integration, and doc-tests (`cargo test`) on every commit.
- **Binary Releases:** Leverage GitHub Actions to automatically cross-compile binaries for multiple targets (Windows, Linux, macOS) on tag releases and publish packages/schemas to registries.
