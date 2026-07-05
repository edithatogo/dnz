# Registry Submission Status

This directory tracks live registry submission metadata for `dnz`.

The current release is installable from GitHub release assets, but the MCP server is not yet publishable to all MCP directories because the available artifact is a local stdio binary. The official MCP Registry and Smithery both need either a supported package/bundle path or a public HTTP endpoint before live submission can be completed.

## Current Status

| Target | Status | Notes |
| --- | --- | --- |
| GitHub Release | Published | `v0.1.0` includes CLI and MCP binaries for Linux, macOS, and Windows. |
| crates.io | Ready for live publish decision | Dry-run checks exist; live publishing needs maintainer credentials and final crate ownership decision. |
| PyPI | Ready for live publish decision | Wheel metadata checks exist; live publishing needs maintainer credentials. |
| Official MCP Registry | Blocked by supported package/bundle path | The registry hosts metadata pointing to packages or remote servers. `dnz-mcp` needs a supported package path, such as MCPB, PyPI console script, npm wrapper, Docker/OCI image, or public remote endpoint. |
| Smithery | Blocked by MCPB or remote HTTP endpoint | Smithery supports URL publishing for streamable HTTP servers and MCPB bundles for local stdio servers. |
| Glama | Metadata prepared | `glama.json` is present for repository indexing; live listing still depends on Glama crawl/review or manual submission. |
| GitHub MCP Registry / Marketplace | Needs path confirmation | Track as official MCP Registry and GitHub marketplace/curation once the available GitHub path is confirmed. |

## Recommended Next Artifact

Create an MCPB bundle for `dnz-mcp` that wraps the released local stdio binary and declares `DIGITALNZ_API_KEY` as a secret configuration value. That bundle can become the common submission artifact for the official MCP Registry, Smithery, and downstream MCP directories that support local MCP packages.
