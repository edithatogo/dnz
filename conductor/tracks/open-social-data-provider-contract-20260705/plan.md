# Track Plan: Open Social Data Provider Contract

## Linked Work

- Downstream Open Social Data track: `open_social_data/conductor/tracks/digitalnz_provider_20260705/`
- Downstream provider issue: https://github.com/edithatogo/open_social_data/issues/5
- Downstream Gazette issue: https://github.com/edithatogo/open_social_data/issues/4
- Downstream CLI/docs issue: https://github.com/edithatogo/open_social_data/issues/3
- DNZ contract issue: https://github.com/edithatogo/dnz/issues/3

## Phase 1: Boundary and Inventory

- [x] Task: Document the upstream/downstream boundary.
    - [x] Confirm `dnz` owns low-level DigitalNZ API, CLI, MCP, cache, and package surfaces.
    - [x] Confirm `open_social_data` owns curated dataset provider, catalog, quality, and Parquet workflows.
    - [x] List `dnz` CLI commands that map to downstream workflows.
- [x] Task: Inventory stable integration surfaces.
    - [x] Identify `dnz-core` types and functions suitable for direct reuse.
    - [x] Identify CLI JSON outputs suitable for interchange.
    - [x] Identify Gazette export manifest and JSONL fields used by downstream ingestion.

## Phase 2: Compatibility Contract

- [x] Task: Define compatibility expectations.
    - [x] Add a compatibility checklist for CLI JSON, manifest, and record schema changes.
    - [x] Define cache path and credential handling expectations.
    - [x] Define fixture and mock-response expectations for downstream tests.
- [x] Task: Decide integration mode for the first Open Social Data provider.
    - [x] Compare direct `dnz-core` dependency against ingesting `gazette-export` artifacts.
    - [x] Record the recommended first implementation path.
    - [x] Note follow-up work if a new library API is needed.

## Phase 3: Documentation and Issue Alignment

- [x] Task: Add operator-facing documentation.
    - [x] Document when to use `dnz search` versus `open-social-data-cli fetch digitalnz ...`.
    - [x] Document when to use `dnz gazette-export` versus curated `nz_gazette` dataset fetches.
    - [x] Document authentication, cache, and deterministic export behavior.
- [x] Task: Align GitHub issues and Conductor tracks.
    - [x] Link this `dnz` track to the downstream `open_social_data` Track 13 issues.
    - [x] Link the downstream track back to this `dnz` contract issue.
    - [x] Keep issue scope separated between upstream contract work and downstream provider work.

## Phase 4: Validation

- [x] Task: Run lightweight validation for tracking-only changes.
    - [x] Run `git diff --check`.
    - [x] Confirm both repos have open Conductor tracks for DigitalNZ/Open Social Data alignment.
    - [x] Confirm no application code was changed as part of track setup.
