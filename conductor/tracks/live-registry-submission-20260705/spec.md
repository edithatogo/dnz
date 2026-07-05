# Track Specification: Live Registry Submission and Verification

## Overview

Submit the current `dnz` CLI and MCP server to live distribution channels and record verifiable publication evidence.

Earlier registry-related tracks prepared release artifacts, package metadata, and dry-run publishing checks. This track covers actual external submission, review, publication, and verification. A target is not complete until the repository records the submitted artifact, registry URL or review URL, submission date, and any follow-up action required by the registry.

## Functional Requirements

- Maintain a registry submission matrix for each target:
  - GitHub Release and repository metadata.
  - crates.io for Rust crates that should be externally consumable.
  - PyPI for Python bindings if the current package metadata remains release-ready.
  - Official MCP Registry.
  - GitHub MCP Registry or GitHub Marketplace/Copilot distribution path when available.
  - Smithery.
  - Glama.
  - Additional MCP directories that are relevant and credible at submission time.
- Add or update registry-specific manifests before submission, including:
  - MCP Registry `server.json` or equivalent publisher metadata.
  - Smithery metadata such as `smithery.yaml` or Smithery-hosted configuration where required.
  - Glama metadata such as `glama.json` where useful for categorization and environment variable documentation.
  - README badges or install snippets only after publication URLs exist.
- Verify namespace, account, and credential requirements before attempting each live submission.
- Preserve secret safety:
  - Do not commit API keys, registry tokens, OAuth tokens, or private test credentials.
  - Use repository secrets or interactive registry login flows where required.
- Record each submission outcome in the plan:
  - Not started.
  - Blocked by account/credential/manual review.
  - Submitted and awaiting review.
  - Published and verified.
  - Rejected with required fixes.

## Non-Functional Requirements

- Registry metadata must describe `dnz` accurately as a DigitalNZ API CLI plus MCP server.
- The CLI and MCP server must remain installable from GitHub release assets even if package registry submission is pending.
- Publication evidence must be durable and linkable.
- Manual registry work must be tracked as a blocker rather than silently marked complete.
- The submission process must not require live DigitalNZ API credentials in public CI.

## Acceptance Criteria

- A GitHub issue exists for live registry submission and links to this track.
- Each target registry has an explicit status, owner action, and verification URL or blocker.
- At least the current GitHub Release is recorded as published evidence.
- Official MCP Registry, Smithery, and Glama have either verified submission URLs or documented account/metadata blockers.
- Any newly required metadata files are committed and validated before submission.
- The track is not archived until live submission outcomes are recorded for all in-scope targets.

## Out of Scope

- Replacing `dnz` with `open_social_data`.
- Shipping a new feature release unless a registry requires packaging changes.
- Committing registry credentials or private DigitalNZ API keys.
- Claiming a manual-curated registry listing before it is approved or visible.

## Current Registry Notes

- GitHub Release `v0.1.0` is already published and includes CLI/MCP binaries.
- The official MCP Registry uses namespace authentication and publisher metadata.
- Smithery supports publishing MCP servers through its hosted/publish flow.
- Glama accepts open-source MCP server submissions from GitHub repositories and can use `glama.json` metadata.
