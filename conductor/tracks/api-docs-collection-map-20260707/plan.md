# Track Plan: API Docs & Collection Map

## Phase 1: Inventory and Mapping

- [ ] Task: Audit the repo’s API-related documentation sources.
    - [ ] Identify the canonical API documentation entrypoints in `README.md`, `pydnz/`, `digitalnz/`, `docs/`, and the crate docs.
    - [ ] Group the sources into canonical docs, historical notebooks, generated artifacts, and reference material.
    - [ ] Capture the mapping in a repo-local document or generated docs page.
- [ ] Task: Define the major-collections inventory criteria.
    - [ ] Derive the collection selection rules from the checked-in facet outputs and visualisation artifacts.
    - [ ] Decide the threshold for “major” collections by count and partner coverage.
    - [ ] Record the source artifact used for each collection row.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Inventory and Mapping' (Protocol in workflow.md)

## Phase 2: Docs Implementation

- [ ] Task: Add the API documentation map page.
    - [ ] Create a docs page that links the API documentation sources to their repo destinations.
    - [ ] Make the page discoverable from the docs index and the most relevant overview pages.
- [ ] Task: Add the major collections page.
    - [ ] Create a docs page or generated artifact listing the major DigitalNZ collections and partners.
    - [ ] Include the source file reference and any selection notes needed to interpret the list.
- [ ] Task: Update docs navigation and cross-links.
    - [ ] Link the new pages from `docs/src/content/docs/index.md` and any relevant guides.
    - [ ] Ensure the docs structure makes the map easy to find without extra context.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Docs Implementation' (Protocol in workflow.md)

## Phase 3: Validation and Closeout

- [ ] Task: Verify the documentation outputs.
    - [ ] Run docs build or check commands for the Astro/Starlight site.
    - [ ] Confirm the collection inventory is deterministic from the repo-local data.
    - [ ] Confirm the docs links point to the intended canonical pages.
- [ ] Task: Record the track outcomes.
    - [ ] Update `conductor/improvement-backlog.md` or learning notes only if the work changes agent guidance.
    - [ ] Update track metadata with the final status when complete.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Validation and Closeout' (Protocol in workflow.md)
