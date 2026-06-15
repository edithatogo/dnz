# DigitalNZ Integration Hub (Rust Core)

A high-performance, strongly typed, and memory-safe integration hub for accessing New Zealand's digital heritage collections via the DigitalNZ API v3. 

Includes an asynchronous Client library, a command-line interface tool, a Model Context Protocol (MCP) server, and FFI Python bindings.

## Workspace Layout

- **`crates/dnz-core`**: Core library engine mapping JSON models, caching search requests, and handling exponential backoff retries.
- **`crates/dnz-cli`**: Command Line Interface (`dnz`) optimized for pipelines (JSON and Markdown output formatting).
- **`crates/dnz-mcp`**: Stdio JSON-RPC MCP server exposing DigitalNZ search capabilities to LLM agents.
- **`crates/dnz-python`**: Python bindings via PyO3/Maturin to import FFI speeds directly in Jupyter notebooks.
- **`docs`**: Custom Astro documentation portal.

## Quickstart

Run tasks and format environments using **Pixi**:

```bash
# Build the workspace
pixi run build

# Run unit and integration tests
pixi run test

# Check clippy guidelines
pixi run clippy
```

For detailed parameters, review the documentation portal under `/docs` or run the dev server:
```bash
cd docs
npm install
npm run dev
```
