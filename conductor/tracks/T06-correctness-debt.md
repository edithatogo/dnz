# T06 — Existing correctness-debt remediation

Fix cache lifecycle/identity, dedupe data loss, XML escaping, date sorting, citation-style overclaims, hard-coded dataset license/coverage, descriptor/file mismatch, Gazette completion/auth metadata, cosine range, vector dimension/upsert semantics, secure model downloads, recursive dense harvesting, and Python runtime-per-request behavior. Every fix requires a regression test.


## Completion record

Status: in_progress

Evidence: conductor/evidence/2026-07-13-t01-t06-slice.md

Open decisions/blockers: Remaining export correctness items include atomic publication and manifest/file reconciliation. Dense harvest partition truncation at 1,000 records is removed; vector-store dimension/upsert, secure model-download, Python runtime lifecycle, and truthful schema metadata are now verified.
