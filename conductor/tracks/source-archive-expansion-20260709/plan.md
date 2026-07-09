# Track Plan: Source Archive Expansion

## Phase 1: Source Inventory

- [ ] Task: Confirm archive-worthy source trees.
  - [ ] Verify which checked-in submodules are source-led and should be mirrored.
  - [ ] Record the current archive scope and what is excluded.
  - [ ] Confirm the combined archive naming in repo documentation.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Source Inventory' (Protocol in workflow.md)

## Phase 2: Archive Bundle Expansion

- [ ] Task: Extend the archive workflows.
  - [ ] Include `pydnz/` in the Hugging Face payload.
  - [ ] Include `pydnz/` in the Zenodo payload.
  - [ ] Use a top-level archive README that describes the combined source bundle.
- [ ] Task: Exclude Git metadata from copied source trees.
  - [ ] Prevent `.git` and related submodule metadata from being uploaded.
  - [ ] Keep the source payloads reproducible and readable.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Archive Bundle Expansion' (Protocol in workflow.md)

## Phase 3: Documentation and Closeout

- [ ] Task: Update archive-facing docs.
  - [ ] Clarify the expanded archive scope in `registry/README.md`.
  - [ ] Ensure docs and registry notes still match the workflow behavior.
- [ ] Task: Validate and record outcome.
  - [ ] Run quick checks on the modified packaging scripts.
  - [ ] Note any remaining source trees that are intentionally out of scope.
  - [ ] Update the track metadata when complete.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Documentation and Closeout' (Protocol in workflow.md)
