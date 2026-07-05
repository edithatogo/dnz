---
title: Registry Submission
---

# Registry Submission

`dnz` currently distributes release binaries through GitHub Releases. Live submissions to package and MCP registries are tracked separately from dry-run packaging checks.

## Published

- GitHub Release `v0.1.0`: CLI and MCP binaries for Linux, macOS, and Windows.

## Ready for Maintainer Credentials

- crates.io: Cargo dry-run checks exist.
- PyPI: Python wheel and metadata checks exist.

## MCP Registry Status

The local MCP server is a stdio binary. The official MCP Registry and Smithery require a supported package or bundle artifact, or a public remote MCP endpoint.

The recommended next submission artifact is an MCPB bundle for `dnz-mcp`, with `DIGITALNZ_API_KEY` declared as a required secret configuration value.

## Configuration

The MCP server reads these environment variables:

- `DIGITALNZ_API_KEY`: required for live DigitalNZ API calls.
- `DNZ_CACHE_PATH`: optional SQLite cache path.
- `DNZ_LOG`: optional log level.
- `DNZ_LOG_FORMAT`: optional log format, with `json` enabling structured logs.
