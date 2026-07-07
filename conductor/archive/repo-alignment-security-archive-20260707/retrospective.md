# Track Retrospective: Repository Alignment, Security Gates & Archival

## Track
- Name: Repository Alignment, Security Gates & Archival
- Track folder: `conductor/archive/repo-alignment-security-archive-20260707`
- Completed date: 2026-07-07
- Lead: Codex
- Reviewer: Codex review pass

## Lessons
### lessons_learned
- Prefer repo-owned security gating code over external action dependencies when the same check already exists locally.
- Pinning workflow actions to commit SHAs is straightforward once the adjacent repo pattern is known.
- A tiny unittest suite is enough to cover the alert-filter logic and keep the gate behavior obvious.

### next_check_to_add
- Re-run the archive publication workflows after any metadata or payload-layout change to make sure the mirror and DOI paths still agree.

## Root cause summary
- What failed and why

## Phase retrospectives
### Phase 1
- Outcome: Workflow and archive parity were inventoried against adjacent repos.
- Evidence: `codeql.yml`, `dependency-review.yml`, `hf_metadata.yml`, `zenodo_publish.yml`, `fyi-archive` comparisons.
- Follow-up: Harden the repo-owned security gate and add the missing scorecard workflow.

### Phase 2
- Outcome: The implementation track was scaffolded and added to the active registry.
- Evidence: `conductor/tracks/repo-alignment-security-archive-20260707/`.
- Follow-up: Complete the workflow and test implementation, then archive the track.

### Phase 3
- Outcome: The implementation and validation work completed cleanly.
- Evidence: commit `3ad511f`, `python -m unittest discover -s tests -p 'test_*.py'`.
- Follow-up: Archive the track and remove it from the active tracks registry.

## Repeat-prevention actions
- Action: Keep code scanning checks in a repo-owned script and test the script directly.
- Owner: Codex
- Verification: `python -m unittest discover -s tests -p 'test_*.py'`

## Reviewer sign-off
- Reviewer: Codex
- Reviewed on: 2026-07-07
- Sign-off status: Approved
- Notes: No open implementation issues remained after the workflow hardening pass.

## Shared artifact updates
- Template updates: None
- Skill updates: None
- Schema updates: None
- Workflow updates: Added a pinned scorecard workflow, pinned dependency review, and repo-owned code scanning gate
