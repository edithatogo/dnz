# T09 evidence-pack and search-provenance evidence

## Implemented slice

- Added deterministic `EvidencePack`, `EvidenceItem`, and `SearchProvenance` types.
- Evidence items retain record IDs, stable citation keys, source/landing URLs, rights statements, and rights URLs.
- Provenance explicitly distinguishes server-side DigitalNZ More Like This from local vector indexes and hybrid composition.
- The builder performs no network fetches and never downloads a model implicitly.
- JSON output is written through a temporary file and published atomically.

## Verification

- `cargo fmt --all -- --check` — PASS.
- `cargo test -p dnz-core evidence_pack_preserves_source_and_distinguishes_api_mlt` — PASS (1 focused test; integration/property binaries clean under the filter).
- `cargo clippy -p dnz-core --all-targets --all-features -- -D warnings` — PASS.

## Remaining T09 work

CSL-compatible citation export or accurately named generic references, lexical/vector/hybrid index workflow integration, and end-to-end evidence-pack consumption remain.
