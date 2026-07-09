# Track Plan: Submission Convergence Loop

## Phase 1: Live Target Inventory

- [x] Task: Audit current submission and verification targets.
  - [x] Check the latest GitHub Actions runs and record which ones are queued, passing, or failing.
  - [x] Check the live registry and release evidence already present in the repo.
  - [x] Capture the current blocker list for each target.
    - CodeQL workflow rerun is queued after disabling default setup.
    - Rust CI/CD Pipeline and Docs runs from the last push were successful.
- [x] Task: Define login checkpoints.
  - [x] Identify which targets need interactive login or refreshed credentials.
  - [x] Record the exact command or web console that requires login.
  - [x] Mark those steps as blocked until credentials are available.
    - No login is currently blocking the repo-local loop.
    - Registry submissions remain dependent on external service state or maintainer review.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Live Target Inventory' (Protocol in workflow.md)

## Phase 2: Convergence Loop

- [x] Task: Apply smallest fixes and re-run targets.
  - [x] Patch repo-local workflow or metadata issues.
  - [x] Re-run the relevant checks after each fix.
  - [x] Continue until the remaining blocker is external or the target is green.
    - Disabled code-scanning default setup so the advanced CodeQL workflow can process SARIF.
    - Re-ran CodeQL against the corrected setup and observed GitHub queue delay.
- [x] Task: Record convergence evidence.
  - [x] Save the final run URLs.
  - [x] Record any remaining auth or external-service blocker.
  - [x] Note when a target reaches its best attainable state.
    - `gh run list --workflow .github/workflows/codeql.yml --branch main --limit 4`
    - `gh api repos/edithatogo/dnz/code-scanning/default-setup`
    - Remaining blocker is GitHub Actions queue backlog, not repo configuration.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Convergence Loop' (Protocol in workflow.md)

## Phase 3: Closeout

- [x] Task: Summarize the result set.
  - [x] State which targets were completed, blocked, or deferred.
  - [x] Link any downstream handoff issue or repo notification.
  - [x] Update the track metadata when complete.
    - Downstream handoff issue: https://github.com/GLAM-Workbench/digitalnz/issues/24
    - Track remains open only on the external GitHub queue; repo-local work is complete.
- [x] Task: Conductor - User Manual Verification 'Phase 3: Closeout' (Protocol in workflow.md)
