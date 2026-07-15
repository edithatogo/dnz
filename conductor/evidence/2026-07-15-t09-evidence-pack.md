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

## Citation and index slice

- Added CSL-JSON-compatible `webpage` references with source URL, publisher, and conservatively parsed year metadata.
- Added deterministic Markdown evidence-pack output; it is labelled as an evidence pack rather than a formal citation style.
- Added an offline lexical token-overlap index with named analyzer/index provenance.
- Added vector-search and equal-weight hybrid adapters over the existing `VectorStore`; model, dimension, index, and component provenance are retained.
- No network access or model download occurs during index construction or search.

## Verification

- `cargo test -p dnz-core evidence::tests::` — PASS (2 tests; filtered integration/property binaries clean).
- `cargo test -p dnz-core research::tests::` — PASS (2 tests; filtered integration/property binaries clean).
- `cargo clippy -p dnz-core --all-targets --all-features -- -D warnings` — PASS.
- `cargo fmt --all -- --check` — PASS.

## Remaining T09 work

End-to-end evidence-pack consumption remains; the scoped citation and offline lexical/vector/hybrid index primitives are implemented.
