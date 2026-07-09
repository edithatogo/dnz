# Track Retrospective: Source Archive Expansion

## Track
- Name: Source Archive Expansion
- Track folder: `conductor/archive/source-archive-expansion-20260709`
- Completed date: 2026-07-09
- Lead: Codex
- Reviewer: Codex review pass

## Lessons
### lessons_learned
- Hugging Face dataset uploads need repo bootstrapping when the target repo does not already exist.
- Zenodo uploads are more reliable when the workflow publishes a single archive artifact plus the top-level README instead of walking nested payload paths.
- Lowercase metadata values matter for Hugging Face README validation.

### next_check_to_add
- Re-run the archive workflows whenever the source bundle scope changes so the HF and Zenodo payload shapes stay aligned.

## Root cause summary
- What failed and why
- The initial archive jobs assumed the downstream repositories already existed and that Zenodo would accept nested file paths from the copied source trees.

## Phase retrospectives
### Phase 1
- Outcome: The archive-worthy source trees were confirmed and the bundle scope was documented.
- Evidence: `digitalnz/`, `pydnz/`, and `rnz/` were identified as the archive payload set.
- Follow-up: Expand the workflows to mirror the full scoped bundle.

### Phase 2
- Outcome: The archive workflows and top-level source archive README were expanded to include the new bundle parts.
- Evidence: commit `fc6ad10`.
- Follow-up: Validate the live publication steps.

### Phase 3
- Outcome: The live Hugging Face and Zenodo archive workflows passed after the workflow fixes.
- Evidence: commits `cbe65ee`, `ef8ac87`; GitHub Actions runs `29014515333` and `29014515440`.
- Follow-up: Archive the track and remove it from the active registry.

## Repeat-prevention actions
- Action: Create or validate downstream archive repos before upload, and keep Zenodo publishing to a single top-level archive artifact.
- Owner: Codex
- Verification: GitHub Actions archive runs

## Reviewer sign-off
- Reviewer: Codex
- Reviewed on: 2026-07-09
- Sign-off status: Approved
- Notes: No open implementation issues remained after the archive publication fixes.

## Shared artifact updates
- Template updates: None
- Skill updates: None
- Schema updates: None
- Workflow updates: Hugging Face repo bootstrapping, lowercased README metadata license, and flattened Zenodo publication payload
