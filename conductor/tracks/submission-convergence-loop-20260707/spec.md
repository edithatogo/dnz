# Track Specification: Submission Convergence Loop

## Overview

Create a repeatable loop for `dnz` submission and publication work so CI, archive, and registry checks can be driven to the best attainable state, with explicit checkpoints for authentication and unresolved external blockers.

This track is about the operational loop, not a single target. It covers the GitHub Actions runs, code-scanning gate, archive publication workflows, and registry verification steps that need iterative retries after fixes land.

## Functional Requirements

- Define the live submission targets and success criteria.
  - GitHub Actions should be monitored until the relevant runs are green or a real blocker is found.
  - Security gates should fail on high/critical findings and report actionable output.
  - Archive publication checks should remain aligned with the checked-in metadata and payload layout.
- Build a convergence loop for submission hardening.
  - Apply the smallest fix that removes a blocker.
  - Re-run the relevant check.
  - Repeat until the target is green or the remaining blocker is external.
- Include explicit authentication checkpoints.
  - Stop and request login when a workflow, registry, or API step needs fresh auth.
  - Record which target requires auth and what credential or login path is needed.
- Keep the loop measurable.
  - Track the target, run URL, status, blocker, and next action in the track plan.
  - Record when a target is “good enough” because the remaining risk is outside repo control.

## Non-Functional Requirements

- The loop must avoid guessing at remote state.
- The loop must not pretend to have completed a submission when it is only queued or blocked by auth.
- The loop should be safe to rerun after each fix.

## Acceptance Criteria

- A Conductor track exists for submission convergence and retry loops.
- The track records the live targets and the conditions under which login is required.
- The track can be used to iterate until the remaining blockers are external rather than repo-local.
- The track can point to the eventual GitHub issue or registry note that records completion status.

## Out of Scope

- Publishing credentials or secret material.
- Inventing new registry targets beyond the ones already relevant to `dnz`.
- Replacing the existing release/submission workflow structure.
