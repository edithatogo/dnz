# T03 — Endpoints, models, and response normalization

Add search, record-by-ID, and More Like This builders; typed common fields plus raw unknown metadata; tolerant verified ID/field/response shapes; structured pagination/facets/request metadata; golden fixtures; and tested error mapping. JSON is stable. XML/RSS remain gated by evidence and secure parsing.


## Completion record

Status: in_progress

Evidence: conductor/evidence/2026-07-14-t03-endpoints-models.md

Open decisions/blockers: RSS parsing remains gated by a stable upstream fixture; the official live RSS endpoint returned HTTP 500 on 2026-07-14. JSON and verified XML endpoint error-shape coverage is complete for the current builders.
