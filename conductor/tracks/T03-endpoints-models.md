# T03 — Endpoints, models, and response normalization

Add search, record-by-ID, and More Like This builders; typed common fields plus raw unknown metadata; tolerant verified ID/field/response shapes; structured pagination/facets/request metadata; golden fixtures; and tested error mapping. JSON is stable. XML/RSS remain gated by evidence and secure parsing.


## Completion record

Status: complete

Evidence: conductor/evidence/2026-07-14-t03-endpoints-models.md

Open decisions/blockers: None in repository implementation. The official live RSS endpoint returned HTTP 500 on 2026-07-15; fixture-backed RSS parsing is covered and live recheck is external follow-up.
