# Track Plan: API Docs & Collection Map

## Phase 1: Inventory and Mapping

- [x] Task: Audit the repo’s API-related documentation sources.
    - [x] Identify the canonical API documentation entrypoints in `README.md`, `pydnz/`, `digitalnz/`, `docs/`, and the crate docs.
    - [x] Group the sources into canonical docs, historical notebooks, generated artifacts, and reference material.
    - [x] Capture the mapping in a repo-local document or generated docs page.
- [x] Task: Define the major-collections inventory criteria.
    - [x] Derive the collection selection rules from the checked-in facet outputs and visualisation artifacts.
    - [x] Decide the threshold for “major” collections by count and partner coverage.
    - [x] Record the source artifact used for each collection row.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Inventory and Mapping' (Protocol in workflow.md)

## Phase 2: Docs Implementation

- [x] Task: Add the API documentation map page.
    - [x] Create a docs page that links the API documentation sources to their repo destinations.
    - [x] Make the page discoverable from the docs index and the most relevant overview pages.
- [x] Task: Add the major collections page.
    - [x] Create a docs page or generated artifact listing the major DigitalNZ collections and partners.
    - [x] Include the source file reference and any selection notes needed to interpret the list.
- [x] Task: Update docs navigation and cross-links.
    - [x] Link the new pages from `docs/src/content/docs/index.md` and any relevant guides.
    - [x] Ensure the docs structure makes the map easy to find without extra context.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Docs Implementation' (Protocol in workflow.md)

## Phase 3: Validation and Closeout

- [x] Task: Verify the documentation outputs.
    - [x] Run docs build or check commands for the Astro/Starlight site.
    - [x] Confirm the collection inventory is deterministic from the repo-local data.
    - [x] Confirm the docs links point to the intended canonical pages.
- [x] Task: Record the track outcomes.
    - [x] Update `conductor/improvement-backlog.md` or learning notes only if the work changes agent guidance.
    - [x] Update track metadata with the final status when complete.
    - [x] Commit: `56563cf`
- [x] Task: Conductor - User Manual Verification 'Phase 3: Validation and Closeout' (Protocol in workflow.md)
