# Track Plan: Live Registry Submission and Verification

## GitHub Issue

- Live registry submission: https://github.com/edithatogo/dnz/issues/4

## Registry Submission Matrix

| Target | Status | Required Evidence |
| --- | --- | --- |
| GitHub Release | Published | https://github.com/edithatogo/dnz/releases/tag/v0.1.0 |
| crates.io | Credential blocked | `CARGO_REGISTRY_TOKEN` is not present in this environment |
| PyPI | Credential blocked | `PYPI_API_TOKEN` / `UV_PUBLISH_TOKEN` are not present in this environment |
| Official MCP Registry | Metadata valid; auth blocked | `mcp-publisher validate registry/mcp/server.draft.json` passes; publish failed with expired/invalid Registry JWT |
| GitHub MCP Registry / Marketplace | Needs path confirmation | Track after official MCP/GitHub publication route is confirmed |
| Smithery | Submitted; rejected by registry validation | Authenticated publish returned `400 Invalid input: expected object, received undefined` |
| Glama | Metadata prepared | `glama.json`; listing still requires crawl/review/submission |
| Other credible MCP directories | Not started | Explicit target list with submission outcome |

## Research References

- Official MCP Registry: https://modelcontextprotocol.io/registry/about
- MCP Registry repository: https://github.com/modelcontextprotocol/registry
- Smithery publish docs: https://smithery.ai/docs/build/publish
- Glama MCP FAQ: https://glama.ai/mcp/faq

## Phase 1: Submission Readiness Audit

- [~] Task: Confirm current release artifacts and package metadata.
    - [x] Verify GitHub Release `v0.1.0` assets, checksums, and release notes.
    - [x] Verify crates intended for publication and package metadata.
    - [x] Verify Python package metadata and wheel readiness.
    - [ ] Confirm install snippets for CLI and MCP server.
- [~] Task: Identify registry-specific prerequisites.
    - [x] Determine account/login requirements for official MCP Registry publishing.
    - [x] Determine Smithery account, hosting, and metadata requirements.
    - [x] Determine Glama GitHub submission and `glama.json` metadata requirements.
    - [ ] Determine GitHub MCP Registry or Marketplace path and whether it is manual curation.

## Phase 2: Metadata and Manifest Preparation

- [~] Task: Add MCP Registry metadata.
    - [x] Create draft registry metadata at `registry/mcp/server.draft.json`.
    - [x] Include namespace, repository, version, and required environment variable notes.
    - [x] Replace draft package blocker with an MCPB release asset and SHA-256.
- [~] Task: Add Smithery metadata.
    - [x] Document Smithery local stdio blocker in `registry/smithery/README.md`.
    - [x] Document required environment variables without committing secrets.
    - [x] Add MCPB bundle metadata once a bundle exists.
- [~] Task: Add Glama metadata.
    - [x] Add `glama.json` for repository indexing.
    - [ ] Confirm category, description, license, build command, and server command during manual review/submission.

## Phase 3: Live Submission

- [x] Task: Submit package registries.
    - [x] Publish crates.io artifacts or record the blocker.
    - [x] Publish PyPI artifacts or record the blocker.
- [ ] Task: Submit MCP registries.
    - [x] Publish to the official MCP Registry or record namespace/authentication blocker.
    - [x] Submit to Smithery or record account/hosting blocker.
    - [ ] Submit to Glama or record review/indexing blocker.
    - [ ] Submit or request curation for GitHub MCP Registry / Marketplace if available.

## Phase 4: Verification and Documentation

- [ ] Task: Verify live listings.
    - [ ] Open each published registry listing and confirm command/install instructions work.
    - [ ] Record listing URLs and publication dates in this plan.
    - [ ] Record rejection or review feedback as follow-up issues.
- [ ] Task: Update user-facing documentation.
    - [ ] Add verified install commands and registry badges only for live listings.
    - [ ] Document API key configuration for CLI and MCP use.
    - [ ] Document registry-specific limitations.

## Phase 5: Track Closure

- [ ] Task: Run validation.
    - [ ] Run `git diff --check`.
    - [ ] Run packaging metadata validation scripts if metadata files changed.
    - [ ] Run release/install smoke checks where practical.
- [ ] Task: Close the loop.
    - [ ] Update the GitHub issue with final submission outcomes.
    - [ ] Mark each registry target as published, blocked, or explicitly deferred.
    - [ ] Archive the track only after all in-scope registry outcomes are recorded.
