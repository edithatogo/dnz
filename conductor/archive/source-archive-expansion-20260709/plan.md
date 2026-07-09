# Track Plan: Source Archive Expansion

## Phase 1: Source Inventory

- [x] Task: Confirm archive-worthy source trees.
  - [x] Verify which checked-in submodules are source-led and should be mirrored.
  - [x] Record the current archive scope and what is excluded.
  - [x] Confirm the combined archive naming in repo documentation.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Source Inventory' (Protocol in workflow.md)

## Phase 2: Archive Bundle Expansion

- [x] Task: Extend the archive workflows.
  - [x] Include `pydnz/` in the Hugging Face payload.
  - [x] Include `pydnz/` in the Zenodo payload.
  - [x] Use a top-level archive README that describes the combined source bundle.
- [x] Task: Exclude Git metadata from copied source trees.
  - [x] Prevent `.git` and related submodule metadata from being uploaded.
  - [x] Keep the source payloads reproducible and readable.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Archive Bundle Expansion' (Protocol in workflow.md)

## Phase 3: Documentation, RNZ Bundle, and Closeout

- [x] Task: Update archive-facing docs.
  - [x] Clarify the expanded archive scope in `registry/README.md`.
  - [x] Ensure docs and registry notes still match the workflow behavior.
- [x] Task: Add the RNZ archive bundle.
  - [x] Create the `rnz/` source bundle from checked-in facet exports and associated documents.
  - [x] Mirror `rnz/` in the Hugging Face and Zenodo workflows.
  - [x] Update archive-facing docs so the RNZ bundle is discoverable.
  - Commit: `fc6ad10`
- [x] Task: Validate and record outcome.
  - [x] Run quick checks on the modified packaging scripts.
  - [x] Note any remaining source trees that are intentionally out of scope.
  - [x] Update the track metadata when complete.
  - [x] Dispatch the HF and Zenodo workflows with the required credentials and confirm the live results for the DigitalNZ, pydnz, and RNZ bundles.
  - Commit: `cbe65ee`, `ef8ac87`
- [x] Task: Conductor - User Manual Verification 'Phase 3: Documentation, RNZ Bundle, and Closeout' (Protocol in workflow.md)
