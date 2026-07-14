# DNZ conductor operating model

## Files

- `manifest.yaml`: tracks, dependencies, priorities, outputs, and gates.
- `state.yaml`: resumable current state and verification record.
- `contracts/`: DigitalNZ API, pydnz compatibility, and cross-surface expectations.
- `tracks/`: detailed acceptance and implementation notes.
- `decisions/`: architecture decision records.
- `evidence/`: generated baseline, API snapshot/digest, test, and benchmark evidence.
- `risk-register.yaml`: active risks, controls, and owners.
- `templates/`: track, ADR, and handoff templates.

`manifest.yaml` is the canonical source for track definitions and acceptance
criteria. `state.yaml` is the canonical source for current status, blockers,
verification, and handoff state. `tracks.md` is a compatibility index only.

## Lifecycle

1. **Discover:** inspect code, tests, docs, submodules, packaging, and current official contracts.
2. **Contract:** update machine-readable and human-readable expectations before broad implementation.
3. **Implement:** complete a small vertical slice in `dnz-core` first, then thin adapters.
4. **Verify:** run focused tests and the full relevant gate; retain concise evidence.
5. **Review:** use a read-only reviewer for correctness, security, compatibility, and docs drift.
6. **Record:** update track acceptance, state, progress, changelog, risks, and handoff.

Statuses are `not_started`, `discovery`, `in_progress`, `blocked`, `verification`, `complete`, or `deferred`. `complete` requires evidence. A failed or unavailable check must be recorded, not silently treated as passing.

## Parallelism

Parallelize read-heavy discovery and reviews. Avoid parallel writers touching shared request/model code. The main agent owns integration and state updates.

## Evidence naming

Use UTC date/time and stable names, for example:

- `evidence/2026-07-12-baseline.md`
- `evidence/2026-07-12-digitalnz-openapi.sha256`
- `evidence/2026-07-12-t03-test-results.md`

Do not store tokens, full authenticated request URLs, or sensitive environment dumps.
