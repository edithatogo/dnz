# Implementation plan: Python 3.14-only runtime

## Phase 1: Baseline and policy

- [~] Task: Record current Python constraints, lockfile entries, workflows, scripts, and documentation.
- [ ] Task: Add failing policy checks proving every supported runtime declaration resolves to Python 3.14.
- [ ] Task: Phase Verification & Checkpoint (Refer to workflow.md).

## Phase 2: Runtime migration

- [ ] Task: Change Pixi and Python package metadata to Python 3.14 only.
- [ ] Task: Update workflows, scripts, tooling targets, classifiers, and documentation.
- [ ] Task: Upgrade only dependencies proven incompatible with Python 3.14.
- [ ] Task: Regenerate affected lockfiles.
- [ ] Task: Phase Verification & Checkpoint (Refer to workflow.md).

## Phase 3: Local validation

- [ ] Task: Run runtime-policy and Python binding tests.
- [ ] Task: Run Pixi lock/install and repository tasks.
- [ ] Task: Run Rust formatting, workspace tests, and Clippy with warnings denied.
- [ ] Task: Run packaging and release smoke checks relevant to Python.
- [ ] Task: Record exact results and any bounded exclusions.
- [ ] Task: Phase Verification & Checkpoint (Refer to workflow.md).

## Phase 4: Publication and hosted evidence

- [ ] Task: Reconcile Conductor specification, plan, registry, state, and evidence.
- [ ] Task: Commit only the scoped migration.
- [ ] Task: Push an isolated branch and open a draft PR.
- [ ] Task: Inspect hosted CI and address only migration-related failures.
- [ ] Task: Mark the track complete only after all required evidence passes.
- [ ] Task: Phase Verification & Checkpoint (Refer to workflow.md).
