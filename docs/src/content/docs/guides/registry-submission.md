---
title: Registry Submission
---

# Registry Submission

`dnz` currently distributes release binaries through GitHub Releases. Live submissions to package and MCP registries are tracked separately from dry-run packaging checks.

The source archive workflows also mirror the checked-in `digitalnz/` notebook corpus, the legacy `pydnz/` Python client source tree, and the `rnz/` archive bundle into Hugging Face and Zenodo source snapshots.

## Published

- GitHub Release `v0.1.0`: CLI and MCP binaries for Linux, macOS, and Windows.
- Install CLI from release assets: download the appropriate `dnz-*` binary for your platform from [v0.1.0](https://github.com/edithatogo/dnz/releases/tag/v0.1.0) and place it on your `PATH`.
- Install MCP from release assets: download `dnz-mcp-0.1.0.mcpb` from [v0.1.0](https://github.com/edithatogo/dnz/releases/tag/v0.1.0) and use it with MCPB-compatible clients.

## Ready for Maintainer Credentials

- crates.io: Cargo dry-run checks exist.
- PyPI: Python wheel and metadata checks exist.

## MCP Registry Status

The local MCP server is a stdio binary. The official MCP Registry and Smithery require a supported package or bundle artifact, or a public remote MCP endpoint.

Build the MCPB bundle:

```powershell
pixi run mcpb
```

The bundle wraps the released `dnz-mcp` binaries and declares `DIGITALNZ_API_KEY` as a required secret configuration value.

The `v0.1.0` bundle is published at:

- https://github.com/edithatogo/dnz/releases/download/v0.1.0/dnz-mcp-0.1.0.mcpb
- SHA-256: `c06f3c4da99b24d3d70545df2e4c802f9d4ecbdb7f4323991d78d104deb41ee6`

The MCP Registry metadata validates with `mcp-publisher validate`. Live publication currently requires a fresh MCP Registry login.

Smithery publication was attempted with the MCPB bundle and reached the registry, but Smithery returned a validation error. Track the live outcome in GitHub issue `#4`.

## Glama And GitHub MCP

Glama accepts open-source MCP servers directly from a GitHub repository. `dnz` has the minimal `glama.json` metadata file and a registry note at `registry/glama/README.md`, but live Glama review remains a manual step.

No distinct public GitHub MCP Registry or Marketplace submission flow was confirmed, so that path remains deferred pending a documented GitHub process.

## Configuration

The MCP server reads these environment variables:

- `DIGITALNZ_API_KEY`: required for live DigitalNZ API calls.
- `DNZ_CACHE_PATH`: optional SQLite cache path.
- `DNZ_LOG`: optional log level.
- `DNZ_LOG_FORMAT`: optional log format, with `json` enabling structured logs.
