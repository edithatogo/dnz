# Track Plan: Submission Convergence Loop

## Phase 1: Live Target Inventory

- [ ] Task: Audit current submission and verification targets.
  - [ ] Check the latest GitHub Actions runs and record which ones are queued, passing, or failing.
  - [ ] Check the live registry and release evidence already present in the repo.
  - [ ] Capture the current blocker list for each target.
- [ ] Task: Define login checkpoints.
  - [ ] Identify which targets need interactive login or refreshed credentials.
  - [ ] Record the exact command or web console that requires login.
  - [ ] Mark those steps as blocked until credentials are available.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Live Target Inventory' (Protocol in workflow.md)

## Phase 2: Convergence Loop

- [ ] Task: Apply smallest fixes and re-run targets.
  - [ ] Patch repo-local workflow or metadata issues.
  - [ ] Re-run the relevant checks after each fix.
  - [ ] Continue until the remaining blocker is external or the target is green.
- [ ] Task: Record convergence evidence.
  - [ ] Save the final run URLs.
  - [ ] Record any remaining auth or external-service blocker.
  - [ ] Note when a target reaches its best attainable state.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Convergence Loop' (Protocol in workflow.md)

## Phase 3: Closeout

- [ ] Task: Summarize the result set.
  - [ ] State which targets were completed, blocked, or deferred.
  - [ ] Link any downstream handoff issue or repo notification.
  - [ ] Update the track metadata when complete.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Closeout' (Protocol in workflow.md)
