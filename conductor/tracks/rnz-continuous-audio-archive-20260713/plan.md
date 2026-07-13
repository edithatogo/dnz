# Track Plan: Zero-Cost Continuous RNZ Audio Archive

## Phase 1: Policy and Contracts

- [~] Task: Implement zero-cost policy and account-audit contracts.
  - [ ] Reject paid runner labels and paid Hugging Face service commands.
  - [ ] Require public repository and personal public-dataset publication.
  - [ ] Document verifiable and manual account safeguards.
- [ ] Task: Define RNZ rights, source, manifest, and output schemas.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Policy and Contracts' (Protocol in workflow.md)

## Phase 2: Resumable Processing

- [ ] Task: Implement DigitalNZ discovery and rights/domain filtering.
- [ ] Task: Implement safe streaming download, checksum deduplication, and retry state.
- [ ] Task: Implement CPU transcription, alignment, diarization, and canonical outputs.
- [ ] Task: Implement WebDataset/Parquet archive assembly and capacity checks.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Resumable Processing' (Protocol in workflow.md)

## Phase 3: Continuous Publication

- [ ] Task: Add daily pilot/backfill orchestration on standard GitHub runners.
- [ ] Task: Add public Hugging Face incremental publication.
- [ ] Task: Add changed-content-only monthly Zenodo publication.
- [ ] Task: Add fail-closed issue reporting and workflow summaries.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Continuous Publication' (Protocol in workflow.md)

## Phase 4: Verification and Rollout

- [ ] Task: Add offline unit, integration, policy, and archive round-trip tests.
- [ ] Task: Verify security gates, formatting, tests, and workflow syntax.
- [ ] Task: Audit account configuration and trigger the bounded pilot when prerequisites pass.
- [ ] Task: Review fixes, document residual blockers, and archive the track.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Verification and Rollout' (Protocol in workflow.md)
