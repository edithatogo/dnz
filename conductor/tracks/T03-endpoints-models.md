# T03 — Endpoints, models, and response normalization

Add search, record-by-ID, and More Like This builders; typed common fields plus raw unknown metadata; tolerant verified ID/field/response shapes; structured pagination/facets/request metadata; golden fixtures; and tested error mapping. JSON is stable. XML/RSS remain gated by evidence and secure parsing.


## Completion record

Status: complete

Evidence: conductor/evidence/2026-07-14-t03-endpoints-models.md

Open decisions/blockers: None in repository implementation. The official live RSS endpoint returned HTTP 500 on 2026-07-15; fixture-backed RSS parsing is covered and live recheck is external follow-up.

## Follow-up Serde/format decision — 18 November 2026 (does not reopen completed T03)

The Rust workspace already uses `serde` derive and `serde_json` for typed common DigitalNZ API fields and response records. `quick-xml` parses XML/RSS; `nutype` enforces domain value constraints; Arrow/Parquet/Polars implement columnar processing. These components are complementary. The verified T03 model explicitly preserves raw, unknown metadata: do **not** apply unconditional `deny_unknown_fields` or convert provider extensions to closed DTOs. A Serde derive does not replace validation, HTTP transport, XML recovery or API licensing.

Conditional checks for a future T03/T10 maintenance increment (new implementation evidence required):
- [ ] Pin official JSON/RSS fixtures, test duplicate keys, unknown-provider fields, mixed-type metadata, unicode, time/number precision, large inputs and unknown-version errors.
- [ ] Verify the typed DTO+raw metadata projection is lossless against installed Python/MCP/CLI and export consumers; only replace manual conversion where a specific duplicate path is demonstrated.
- [ ] Preserve separate live RSS 500/provider gate and metadata rights claims; do not claim it passed from typed-fixture success.
- [ ] Gate any concrete migration on locked workspace build, feature/target profiles, exact-head consumer tests and unchanged public wire semantics. No new Serde migration is recommended at present.
