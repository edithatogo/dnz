# T02 — Search query completeness

Implement the complete verified search parameter contract with a safe query/filter representation, exact `per_page=0`, current facet limits, nested boolean filters, bbox validation, safe extra parameters, and canonical serialization. Add encoding/property tests for Unicode, repeated values, reserved characters, invalid ranges, and protected parameters.


## Completion record

Status: blocked

Evidence: conductor/evidence/2026-07-13-t01-t06-slice.md

Open decisions/blockers: Focused executable tests and complete serializer verification remain; retain the partial implementation until CI or a repaired toolchain verifies it.
