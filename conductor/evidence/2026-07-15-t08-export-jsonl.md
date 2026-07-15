# T08 deterministic JSONL export evidence

## Implemented slice

- Added `write_records_jsonl`, a reusable normalized-record JSONL writer with parent creation, deterministic record order supplied by the caller, flush, and atomic publish.
- Existing Gazette exports continue to use atomic temporary files and manifest reconciliation.

## Verification

- `cargo fmt --all -- --check` — PASS.
- `cargo test -p dnz-core export::tests::records_jsonl_is_deterministic_and_atomic` — PASS.
- Full core gate — PASS (76 unit, 12 integration, 5 property, 0 doctest failures).
- Clippy with warnings denied — PASS.

## CSV slice

- Added `write_records_csv` with an explicit stable schema, RFC-style quoting, deterministic row order, and formula-like value neutralization for spreadsheet safety.
- JSONL remains the lossless export path for unknown provider fields; CSV is an intentional tabular projection.
- Full core gate after this slice: 77 unit, 12 integration, 5 property, 0 doctest failures; Clippy passes with warnings denied.

## GeoJSON slice

- Added `write_records_geojson`, emitting only finite WGS84 points from common provider location shapes and preserving ID/title/source/rights properties.
- Invalid or absent coordinates are omitted rather than guessed; nested location objects and coordinate arrays are supported.
- Full core gate after this slice: 78 unit, 12 integration, 5 property, 0 doctest failures; Clippy passes.

## Provenance and checksum slice

- Added `ExportProvenance` descriptors with schema version, source URL, record count, deterministic file checksums, and explicit limitations.
- The checksum algorithm is named `fnv1a64` and is documented as change detection only, not cryptographic authenticity.
- Full core gate after this slice: 79 unit, 12 integration, 5 property, 0 doctest failures; Clippy passes.

## Remaining T08 work

Parquet/Arrow, data-quality metrics, and rights/reuse summaries remain.

## Remaining T08 work

CSV/Parquet or Arrow export, validated geospatial formats, checksums/schema/provenance expansion, data-quality metrics, and rights/reuse summaries remain.
