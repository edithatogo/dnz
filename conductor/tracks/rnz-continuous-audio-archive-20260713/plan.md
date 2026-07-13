# Track Plan: Zero-Cost Continuous RNZ Audio Archive

## Phase 1: Policy and Contracts

- [x] Task: Implement zero-cost policy and account-audit contracts (`d835671`, `f67c00e`).
  - [x] Reject paid runner labels and paid Hugging Face service commands.
  - [x] Require public repository and personal public-dataset publication.
  - [x] Document verifiable and manual account safeguards.
- [x] Task: Define RNZ rights, source, manifest, and output schemas (`d835671`).
- [x] Task: Conductor - User Manual Verification 'Phase 1: Policy and Contracts' (Protocol in workflow.md)

## Phase 2: Resumable Processing

- [x] Task: Implement DigitalNZ discovery and rights/domain filtering (`d835671`, `f67c00e`).
- [x] Task: Implement safe streaming download, checksum deduplication, and retry state (`d835671`).
- [x] Task: Implement CPU transcription, alignment, diarization, and canonical outputs (`d835671`, `6c1672e`).
- [x] Task: Implement WebDataset/Parquet archive assembly and capacity checks (`d835671`, `f67c00e`).
- [x] Task: Conductor - User Manual Verification 'Phase 2: Resumable Processing' (Protocol in workflow.md)

## Phase 3: Continuous Publication

- [x] Task: Add daily pilot/backfill orchestration on standard GitHub runners (`d835671`).
- [x] Task: Add public Hugging Face incremental publication (`d835671`).
- [x] Task: Add changed-content-only monthly Zenodo publication (`d835671`, `f67c00e`).
- [x] Task: Add fail-closed issue reporting and workflow summaries (`f67c00e`).
- [x] Task: Conductor - User Manual Verification 'Phase 3: Continuous Publication' (Protocol in workflow.md)

## Phase 4: Verification and Rollout

- [x] Task: Add offline unit, integration, policy, and archive round-trip tests (`d835671`, `6c1672e`).
- [x] Task: Verify security gates, formatting, tests, and workflow syntax.
- [~] Task: Audit account configuration and trigger the bounded pilot when prerequisites pass.
  - [x] Verify GitHub public/free-runner and Hugging Face zero-credit/free-storage state.
  - [x] Configure fail-closed repository variables and production environment.
  - [x] Accept gated pyannote Community-1 access with explicit user consent (2026-07-13).
  - [x] Verify the Actions token can read the gated model (Actions run `29244743661`).
  - [~] Set `RNZ_ARCHIVE_ENABLED=true` and run the bounded pilot.
    - [x] Enable the fail-closed production gate and start exact-record smoke run `29247010802`.
    - [x] Verify non-empty Hugging Face round trip after CPU transcription completes (run `29247010802`).
    - [x] Exercise the enrichment proof path on the same bounded record; run `29249683277` exposed and led to the no-op/reprocess fix in `00c8a09`.
    - [x] Receive repository-owner acceptance of the smoke output and submit authenticated `approved` disposition workflow `29250061230` (2026-07-13).
    - [ ] Resubmit the corrected exact-record reprocess proof after the active backfill and approval ledger runs complete; queued run `29249874617` was replaced by GitHub's one-pending concurrency rule.
    - [ ] Run and manually review the stratified 100-record pilot.
- [ ] Task: Review fixes, document residual blockers, and archive the track.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Verification and Rollout' (Protocol in workflow.md)
