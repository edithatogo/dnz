# Track Plan: Open Social Data Provider Contract

## Linked Work

- Downstream Open Social Data track: `open_social_data/conductor/tracks/digitalnz_provider_20260705/`
- Downstream provider issue: https://github.com/edithatogo/open_social_data/issues/5
- Downstream Gazette issue: https://github.com/edithatogo/open_social_data/issues/4
- Downstream CLI/docs issue: https://github.com/edithatogo/open_social_data/issues/3
- DNZ contract issue: https://github.com/edithatogo/dnz/issues/3

## Phase 1: Boundary and Inventory

- [ ] Task: Document the upstream/downstream boundary.
    - [ ] Confirm `dnz` owns low-level DigitalNZ API, CLI, MCP, cache, and package surfaces.
    - [ ] Confirm `open_social_data` owns curated dataset provider, catalog, quality, and Parquet workflows.
    - [ ] List `dnz` CLI commands that map to downstream workflows.
- [ ] Task: Inventory stable integration surfaces.
    - [ ] Identify `dnz-core` types and functions suitable for direct reuse.
    - [ ] Identify CLI JSON outputs suitable for interchange.
    - [ ] Identify Gazette export manifest and JSONL fields used by downstream ingestion.

## Phase 2: Compatibility Contract

- [ ] Task: Define compatibility expectations.
    - [ ] Add a compatibility checklist for CLI JSON, manifest, and record schema changes.
    - [ ] Define cache path and credential handling expectations.
    - [ ] Define fixture and mock-response expectations for downstream tests.
- [ ] Task: Decide integration mode for the first Open Social Data provider.
    - [ ] Compare direct `dnz-core` dependency against ingesting `gazette-export` artifacts.
    - [ ] Record the recommended first implementation path.
    - [ ] Note follow-up work if a new library API is needed.

## Phase 3: Documentation and Issue Alignment

- [ ] Task: Add operator-facing documentation.
    - [ ] Document when to use `dnz search` versus `open-social-data-cli fetch digitalnz ...`.
    - [ ] Document when to use `dnz gazette-export` versus curated `nz_gazette` dataset fetches.
    - [ ] Document authentication, cache, and deterministic export behavior.
- [ ] Task: Align GitHub issues and Conductor tracks.
    - [ ] Link this `dnz` track to the downstream `open_social_data` Track 13 issues.
    - [ ] Link the downstream track back to this `dnz` contract issue.
    - [ ] Keep issue scope separated between upstream contract work and downstream provider work.

## Phase 4: Validation

- [ ] Task: Run lightweight validation for tracking-only changes.
    - [ ] Run `git diff --check`.
    - [ ] Confirm both repos have open Conductor tracks for DigitalNZ/Open Social Data alignment.
    - [ ] Confirm no application code was changed as part of track setup.
