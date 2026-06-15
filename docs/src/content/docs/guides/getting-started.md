---
title: Getting Started
description: Overview and quickstart guide for the DigitalNZ Rust Hub.
---

The **DigitalNZ Integration Hub** is a native, high-performance toolkit written in Rust, designed to connect New Zealand's digital heritage archives to modern software layers, CLI terminals, and AI developer engines.

## Installation & Setup

Ensure you have [Rust](https://rustup.rs/) and [Pixi](https://pixi.sh/) installed.

### 1. Configure API Key
Export your DigitalNZ API key into your shell profile environment:
```bash
export DIGITALNZ_API_KEY="your-api-key-here"
```

### 2. Build the Workspace
To build all binary utilities, library crates, and test targets concurrently, run:
```bash
pixi run build
```

---

## Using the CLI (`dnz-cli`)

Run search queries formatting output directly as Markdown tables or structured JSON:
```bash
# Markdown table output
cargo run --bin dnz-cli -- search "kiwi" --format markdown

# Output formatted JSON string
cargo run --bin dnz-cli -- search "kiwi" --format json
```

Query collection facet distributions:
```bash
cargo run --bin dnz-cli -- facets "tui" --fields category,collection --format markdown
```

---

## Starting the MCP Server (`dnz-mcp`)

Run the Model Context Protocol server over standard I/O (stdio) to interact with AI clients (e.g. Claude Desktop, Cline):
```bash
cargo run --bin dnz-mcp
```
*(Stderr logs trace initialization events while stdin/stdout remains dedicated to structured JSON-RPC messages).*
