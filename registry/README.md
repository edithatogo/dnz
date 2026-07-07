# Registry Submission Status

This directory tracks live registry submission metadata for `dnz`.

The current release is installable from GitHub release assets, but the MCP server is not yet publishable to all MCP directories because the available artifact is a local stdio binary. The official MCP Registry and Smithery both need either a supported package/bundle path or a public HTTP endpoint before live submission can be completed.

## Current Status

| Target | Status | Notes |
| --- | --- | --- |
| GitHub Release | Published | `v0.1.0` includes CLI and MCP binaries for Linux, macOS, and Windows. |
| crates.io | Credential blocked | Dry-run checks exist; `CARGO_REGISTRY_TOKEN` is not present in this environment. |
| PyPI | Credential blocked | Wheel metadata checks exist; `PYPI_API_TOKEN` / `UV_PUBLISH_TOKEN` are not present in this environment. |
| Official MCP Registry | Metadata valid; auth blocked | `mcp-publisher validate registry/mcp/server.draft.json` passes. Live publish needs a fresh MCP Registry login. |
| Smithery | Submitted; rejected by registry validation | Authenticated publish reached Smithery, but the registry returned `400 Invalid input: expected object, received undefined`. |
| Glama | Metadata prepared | `glama.json` is present for repository indexing; live listing still depends on Glama crawl/review or manual submission. |
| GitHub MCP Registry / Marketplace | Needs path confirmation | Track as official MCP Registry and GitHub marketplace/curation once the available GitHub path is confirmed. |

## Archive Mirrors

The notebook archive under `digitalnz/` now has workflow-backed metadata paths for public mirrors:

| Target | Status | Notes |
| --- | --- | --- |
| Hugging Face | Metadata workflow configured | `digitalnz/DATASET_CARD.md` is staged by `.github/workflows/hf_metadata.yml` and can be uploaded once `HF_TOKEN` and `HF_REPO_ID` are set. |
| Zenodo | Metadata workflow configured | `digitalnz/.zenodo.json` is staged by `.github/workflows/zenodo_publish.yml`; live deposition publication still requires a Zenodo token and manual confirmation. |

## Recommended Next Artifact

Build the MCPB bundle:

```powershell
pixi run mcpb
```

The bundle wraps the released local stdio binaries and declares `DIGITALNZ_API_KEY` as a secret configuration value.

Published bundle:

- URL: https://github.com/edithatogo/dnz/releases/download/v0.1.0/dnz-mcp-0.1.0.mcpb
- SHA-256: `c06f3c4da99b24d3d70545df2e4c802f9d4ecbdb7f4323991d78d104deb41ee6`
