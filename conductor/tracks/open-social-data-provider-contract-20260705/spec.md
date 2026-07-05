# Track Specification: Open Social Data Provider Contract

## Overview

Define and maintain the `dnz` side of the DigitalNZ integration contract for `open_social_data`.

`dnz` remains the low-level DigitalNZ API client, CLI, MCP server, Python/Rust package surface, cache implementation, and ad hoc research tool. `open_social_data` should consume stable `dnz` capabilities for curated dataset workflows, especially New Zealand Gazette ingestion, cataloging, quality checks, and Parquet export.

This track exists so future `dnz` CLI and core-library changes are explicitly evaluated for downstream `open_social_data` provider compatibility.

## Functional Requirements

- Document which `dnz` capabilities are stable integration surfaces:
  - `dnz-core` search query builder and response models.
  - `dnz-core` persistent cache configuration.
  - Gazette export manifest and JSONL record layout.
  - CLI JSON outputs for `search`, `facets`, and `gazette-export`.
  - Authentication through `DIGITALNZ_API_KEY` and `--api-key`.
- Define which capabilities should remain `dnz`-only:
  - Ad hoc search and facet exploration.
  - MCP server interaction.
  - Python bindings and notebook workflows.
  - Workspace diagnostics through `dnz doctor`.
- Add compatibility guidance for `open_social_data`:
  - Preferred direct dependency or interchange artifact for each workflow.
  - Versioning expectations for exported records and manifests.
  - Cache and credential handling requirements.
  - Deterministic fixture strategy for CI.
- Track upstream changes needed by the `open_social_data` DigitalNZ provider.

## Non-Functional Requirements

- The contract must avoid leaking API keys into logs, cache records, manifests, or fixture files.
- Integration outputs should be deterministic enough for fixture tests and data catalog updates.
- Contract documentation must be clear enough for future `dnz` CLI changes to identify whether they are breaking downstream provider behavior.
- `dnz` must remain independently useful and releasable.

## Acceptance Criteria

- A GitHub issue exists in `dnz` for the Open Social Data provider contract.
- The `open_social_data` DigitalNZ provider track links back to the `dnz` contract issue or track.
- The contract identifies the stable `dnz` surfaces that `open_social_data` may consume.
- Future Gazette export or JSON output changes have a documented compatibility checklist.
- The plan distinguishes implementation work in `dnz` from downstream provider work in `open_social_data`.

## Out of Scope

- Moving the `dnz` CLI into `open_social_data`.
- Replacing `open_social_data` provider, catalog, validation, or Parquet logic.
- Implementing the DigitalNZ provider in `open_social_data`.
- Publishing new `dnz` releases.
