# Findings & Scratchpad

## Workspace Discoveries
- **Workspace root:** `dnz` configured as a multi-crate Cargo workspace.
- **Member Crates:**
  - `dnz-core`: Native library mapping client endpoints and data handling.
  - `dnz-cli`: Command Line Interface (`dnz` command).
  - `dnz-mcp`: Model Context Protocol server.
- **Environments:** Managed via `pixi` for cross-platform system libraries (openssl, cross-compilers).
- **Core target:** Port DigitalNZ API client to Rust using `reqwest` and `serde`.

## Advanced Features Reference
- **Semantic Vector:** Embeddings generated using `candle` offline model loader.
- **Query Autopilot:** Dynamic query splitting using facet metrics to fetch complete deep datasets.
- **FFI:** Python FFI bindings using `pyo3` and `maturin`.
