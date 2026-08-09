# Implementation plan: Python 3.14-only runtime

## Phase 1: Baseline and policy

- [x] Task: Record current Python constraints, lockfile entries, workflows, scripts, and documentation. `09f0193`
- [x] Task: Add failing policy checks proving every supported runtime declaration resolves to Python 3.14. `d9f0aed`
- [x] Task: Phase Verification & Checkpoint (Refer to workflow.md). Red gate reproduced with 12 policy violations.

## Phase 2: Runtime migration

- [x] Task: Change Pixi and Python package metadata to Python 3.14 only. `bbf9725`
- [x] Task: Update workflows, scripts, tooling targets, classifiers, and documentation. `bbf9725`
- [x] Task: Upgrade only dependencies proven incompatible with Python 3.14. `b55639f`
- [x] Task: Regenerate affected lockfiles. `bbf9725`, `b55639f`
- [x] Task: Phase Verification & Checkpoint (Refer to workflow.md). Policy and targeted tests passed.

## Phase 3: Local validation

- [x] Task: Run runtime-policy and Python binding tests. 44 Python tests passed in the locked RNZ environment.
- [x] Task: Run Pixi lock/install and repository tasks. `pixi install --locked` and `pixi run verify-local` passed.
- [x] Task: Run Rust formatting, workspace tests, and Clippy with warnings denied. Passed via `verify-local`.
- [x] Task: Run packaging and release smoke checks relevant to Python. `3f3c027`; CPython 3.14 wheel build, Twine metadata, install, and import passed.
- [x] Task: Record exact results and any bounded exclusions. See `conductor/evidence/2026-08-10-python-314-validation.md`.
- [x] Task: Phase Verification & Checkpoint (Refer to workflow.md). Local required gates passed; coverage explicitly deferred to hosted CI by the repository's Windows prerequisite gate.

## Phase 4: Publication and hosted evidence

- [x] Task: Reconcile Conductor specification, plan, registry, state, and evidence.
- [x] Task: Commit only the scoped migration. Task-level commits recorded above.
- [x] Task: Push an isolated branch and open a draft PR. PR #43.
- [x] Task: Inspect hosted CI and address only migration-related failures. All required checks passed without migration-related failures.
- [x] Task: Mark the track complete only after all required evidence passes.
- [x] Task: Phase Verification & Checkpoint (Refer to workflow.md). Hosted Python analysis, dependency review, documentation, quality/coverage, and CodeQL checks passed.
