# Track Retrospective: Submission Convergence Loop

## Track
- Name: Submission Convergence Loop
- Track folder: `conductor/archive/submission-convergence-loop-20260707`
- Completed date: 2026-07-09
- Lead: Codex
- Reviewer: Codex review pass

## Lessons
### lessons_learned
- Treat submission work as a retryable loop with explicit external-blocker detection rather than a one-shot release step.
- Keep authentication checkpoints separate from repo-local failures so the loop can stop cleanly when human login is required.
- Record the live target URL and blocker state in the track itself so future reruns do not need the full chat history.

### next_check_to_add
- Re-run the submission loop whenever a workflow, registry, or archive publication step changes upstream behavior.

## Root cause summary
- What failed and why
- Submission and publication targets can drift independently from repo-local code, so convergence needs explicit retry and blocker tracking instead of assuming queued work will settle on its own.

## Phase retrospectives
### Phase 1
- Outcome: The live target inventory and login checkpoints were recorded.
- Evidence: `gh run list`, registry/release checks, and the track plan notes.
- Follow-up: Keep the target list current as workflows change.

### Phase 2
- Outcome: Repo-local blockers were cleared and the loop reached the best attainable state.
- Evidence: the CodeQL queue backlog and repo-local fixes recorded in the plan.
- Follow-up: Re-run the relevant checks after future workflow changes.

### Phase 3
- Outcome: The residual state and downstream handoff were documented.
- Evidence: downstream handoff issue `https://github.com/GLAM-Workbench/digitalnz/issues/24`.
- Follow-up: Archive the track and remove it from the active registry.

## Repeat-prevention actions
- Action: Keep convergence loops focused on a small set of live targets and record blocker type explicitly.
- Owner: Codex
- Verification: GitHub Actions and registry state

## Reviewer sign-off
- Reviewer: Codex
- Reviewed on: 2026-07-09
- Sign-off status: Approved
- Notes: No repo-local blockers remained; the only residual state was external or downstream.

## Shared artifact updates
- Template updates: None
- Skill updates: None
- Schema updates: None
- Workflow updates: Added explicit convergence-loop documentation and handoff tracking
