# T01 — Transport, auth, secrets, cache isolation, and errors

Deliver a compatibility-preserving client builder/unauthenticated mode; optional `Authentication-Token` header; explicit legacy query-key mode; HTTPS/configurable API root; redacted request/debug/error surfaces; auth-safe cache namespaces; timeouts; bounded retries honoring `Retry-After`; and structured API/network/decode errors. Tests must prove no credential leakage and no cross-auth cache reuse.


## Completion record

Status: blocked

Evidence: conductor/evidence/2026-07-13-t01-t06-slice.md

Open decisions/blockers: Implementation and regression tests are present, but focused runtime execution and full workspace verification are externally blocked by prolonged dependency compilation/resource contention, plus the known local MSVC/Python environment failures. The archived track remains incomplete until CI or a repaired toolchain executes the tests successfully.
