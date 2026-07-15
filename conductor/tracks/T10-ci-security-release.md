# T10 — CI, security, packaging, release, and performance

Add contract-drift and opt-in live tests, full surface CI, audit/deny/license/secret/SBOM/provenance controls, release archive checks, cross-platform crates/CLI/Python/Pixi/Conda packaging, optional image, MSRV/SemVer/deprecation/release docs, and stable benchmarks.


## Completion record

Status: in_progress

Evidence: conductor/evidence/2026-07-16-t10-hardening.md

Open decisions/blockers: hosted workflow execution remains required evidence. Local audit is green after the optional export dependency was narrowed to polars-core plus Arrow/Parquet; one allowed unmaintained paste warning remains explicitly reported.
