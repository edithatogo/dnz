# T01 — Transport, auth, secrets, cache isolation, and errors

Deliver a compatibility-preserving client builder/unauthenticated mode; optional `Authentication-Token` header; explicit legacy query-key mode; HTTPS/configurable API root; redacted request/debug/error surfaces; auth-safe cache namespaces; timeouts; bounded retries honoring `Retry-After`; and structured API/network/decode errors. Tests must prove no credential leakage and no cross-auth cache reuse.


## Completion record

Status: in_progress

Evidence: conductor/evidence/2026-07-13-t01-t06-slice.md

Open decisions/blockers: Focused runtime tests and full workspace verification remain. Structured errors, bounded Retry-After handling, timeout configuration, HTTPS endpoint checks, auth-mode cache namespaces, and redacted transport diagnostics are implemented in the current slice.
