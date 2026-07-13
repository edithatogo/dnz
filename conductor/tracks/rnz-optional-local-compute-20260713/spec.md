# Track Specification: Optional Local RNZ Compute

## Goal

After the zero-cost hosted archive and its current enrichment tracks are complete, allow explicitly invoked processing on user-owned hardware for workloads that are impractical on standard GitHub runners.

## Requirements

- Keep this track blocked behind completion and review of Tracks 22 through 26.
- Use only user-owned local CPU, GPU, memory and disk; external service spend remains USD 0.
- Never call paid cloud compute, inference, storage, databases, APIs or hosted GPU services.
- Require manual opt-in, resource ceilings, resumable checkpoints and a dry-run mode.
- Preserve canonical hosted outputs and publish local results only as versioned, checksummed optional derivatives after validation.
- Record hardware, software, model revisions, runtime, energy-relevant runtime metrics and provenance.
- Candidate workloads include corpus-wide perceptual matching, larger alignment experiments, acoustic evaluation, local entity linking and local retrieval indexes.

## Acceptance Criteria

- The hosted archive remains complete and operable when local processing never runs.
- No scheduled GitHub workflow invokes or waits for local hardware.
- Static policy tests reject paid service credentials and commands in the local workflow.
- Interrupted work resumes without recomputing completed outputs.
- Every promoted derivative passes schema, integrity, rights and review gates.
