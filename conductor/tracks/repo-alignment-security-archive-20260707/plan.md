# Track Plan: Repository Alignment, Security Gates & Archival

## Phase 1: Inventory and Parity Map

- [x] Task: Audit current workflows and adjacent-repo patterns.
  - [x] Review the repo’s current GitHub Actions, permissions, and security checks.
  - [x] Compare the archive publication shape against the adjacent repos that already publish archives or dataset metadata.
  - [x] Capture the repo-local Conductor state that the next implementation step must preserve.
- [x] Task: Define the required parity set.
  - [x] Decide which gates must fail PRs for high or critical security findings.
  - [x] Decide which archive publication artifacts and metadata are required.
  - [x] Record the baseline in a track-local note or issue checklist.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Inventory and Parity Map' (Protocol in workflow.md)

## Phase 2: Track Scaffold and Follow-On Hooks

- [x] Task: Write the execution-ready parity plan.
  - [x] Separate repo-local implementation work from any downstream or adjacent repo work.
  - [x] Keep the next track small enough to execute without re-running the inventory.
  - [x] Add any explicit issue/PR backlinks needed for the follow-on work.
- [x] Task: Align the Conductor registry surface.
  - [x] Ensure the new track appears in `conductor/tracks.md`.
  - [x] Keep the track metadata and folder naming aligned with existing completed tracks.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Track Scaffold and Follow-On Hooks' (Protocol in workflow.md)

## Phase 3: Validation and Handoff

- [x] Task: Validate the track bundle.
  - [x] Confirm `spec.md`, `plan.md`, `metadata.json`, and `index.md` are present and consistent.
  - [x] Confirm the track name, description, and status reflect a pending parity implementation.
  - [x] Confirm there is a clear next step for the implementation track.
  - [x] Commit: `3ad511f`
- [x] Task: Conductor - User Manual Verification 'Phase 3: Validation and Handoff' (Protocol in workflow.md)
