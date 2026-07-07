# Track Plan: Repository Alignment, Security Gates & Archival

## Phase 1: Inventory and Parity Map

- [ ] Task: Audit current workflows and adjacent-repo patterns.
  - [ ] Review the repo’s current GitHub Actions, permissions, and security checks.
  - [ ] Compare the archive publication shape against the adjacent repos that already publish archives or dataset metadata.
  - [ ] Capture the repo-local Conductor state that the next implementation step must preserve.
- [ ] Task: Define the required parity set.
  - [ ] Decide which gates must fail PRs for high or critical security findings.
  - [ ] Decide which archive publication artifacts and metadata are required.
  - [ ] Record the baseline in a track-local note or issue checklist.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Inventory and Parity Map' (Protocol in workflow.md)

## Phase 2: Track Scaffold and Follow-On Hooks

- [ ] Task: Write the execution-ready parity plan.
  - [ ] Separate repo-local implementation work from any downstream or adjacent repo work.
  - [ ] Keep the next track small enough to execute without re-running the inventory.
  - [ ] Add any explicit issue/PR backlinks needed for the follow-on work.
- [ ] Task: Align the Conductor registry surface.
  - [ ] Ensure the new track appears in `conductor/tracks.md`.
  - [ ] Keep the track metadata and folder naming aligned with existing completed tracks.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Track Scaffold and Follow-On Hooks' (Protocol in workflow.md)

## Phase 3: Validation and Handoff

- [ ] Task: Validate the track bundle.
  - [ ] Confirm `spec.md`, `plan.md`, `metadata.json`, and `index.md` are present and consistent.
  - [ ] Confirm the track name, description, and status reflect a pending parity implementation.
  - [ ] Confirm there is a clear next step for the implementation track.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Validation and Handoff' (Protocol in workflow.md)
