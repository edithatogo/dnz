# Track Plan: Live Registry Submission and Verification

## GitHub Issue

- Live registry submission: https://github.com/edithatogo/dnz/issues/4

## Registry Submission Matrix

| Target | Status | Required Evidence |
| --- | --- | --- |
| GitHub Release | Published | https://github.com/edithatogo/dnz/releases/tag/v0.1.0 |
| crates.io | Not started | Published crate URLs or documented publish blocker |
| PyPI | Not started | Published package URL or documented publish blocker |
| Official MCP Registry | Not started | Published registry entry or submitted review URL |
| GitHub MCP Registry / Marketplace | Not started | Published listing, curation request, or documented unavailable path |
| Smithery | Not started | Published Smithery listing or submission/review URL |
| Glama | Not started | Published Glama listing or submission/review URL |
| Other credible MCP directories | Not started | Explicit target list with submission outcome |

## Research References

- Official MCP Registry: https://modelcontextprotocol.io/registry/about
- MCP Registry repository: https://github.com/modelcontextprotocol/registry
- Smithery publish docs: https://smithery.ai/docs/build/publish
- Glama MCP FAQ: https://glama.ai/mcp/faq

## Phase 1: Submission Readiness Audit

- [ ] Task: Confirm current release artifacts and package metadata.
    - [ ] Verify GitHub Release `v0.1.0` assets, checksums, and release notes.
    - [ ] Verify crates intended for publication and package metadata.
    - [ ] Verify Python package metadata and wheel readiness.
    - [ ] Confirm install snippets for CLI and MCP server.
- [ ] Task: Identify registry-specific prerequisites.
    - [ ] Determine account/login requirements for official MCP Registry publishing.
    - [ ] Determine Smithery account, hosting, and metadata requirements.
    - [ ] Determine Glama GitHub submission and `glama.json` metadata requirements.
    - [ ] Determine GitHub MCP Registry or Marketplace path and whether it is manual curation.

## Phase 2: Metadata and Manifest Preparation

- [ ] Task: Add MCP Registry metadata.
    - [ ] Create or update registry `server.json` metadata.
    - [ ] Include namespace, repository, version, transport, package, and runtime configuration.
    - [ ] Validate metadata against the current publisher requirements.
- [ ] Task: Add Smithery metadata.
    - [ ] Create or update Smithery configuration if the selected publish path requires it.
    - [ ] Document whether the server is local stdio, hosted, or bundled.
    - [ ] Validate required environment variables without committing secrets.
- [ ] Task: Add Glama metadata.
    - [ ] Add `glama.json` if it improves indexing or environment variable documentation.
    - [ ] Confirm category, description, license, build command, and server command.

## Phase 3: Live Submission

- [ ] Task: Submit package registries.
    - [ ] Publish crates.io artifacts or record the blocker.
    - [ ] Publish PyPI artifacts or record the blocker.
- [ ] Task: Submit MCP registries.
    - [ ] Publish to the official MCP Registry or record namespace/authentication blocker.
    - [ ] Submit to Smithery or record account/hosting blocker.
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
