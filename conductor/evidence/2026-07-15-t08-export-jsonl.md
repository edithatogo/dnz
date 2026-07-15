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

## Data-quality and rights slice

- Added observable `DataQualityReport` metrics for totals, unique/duplicate IDs, missing titles, source URLs, and rights metadata.
- Added `RightsReuseAudit` counts for rights statements, rights URLs, usage statements, and commercial-use flags.
- Both reports explicitly state that they summarize supplied metadata only and are not legal advice or determinations of reuse permission.
- Targeted quality tests pass; Clippy with warnings denied passes.

## Source-grounded packaging slice

- Added RO-Crate 1.1 metadata generation linking the dataset, JSONL distribution, supplied source endpoint, and DigitalNZ publisher identity.
- The package description explicitly limits rights and completeness claims to metadata requiring source-specific review.
- Targeted RO-Crate test passes; formatting and Clippy remain clean.

## Remaining T08 work

Parquet/Arrow remains; source-grounded Frictionless/schema.org/RO-Crate coverage is now implemented for the current export surfaces.

## Remaining T08 work

CSV/Parquet or Arrow export, validated geospatial formats, checksums/schema/provenance expansion, data-quality metrics, and rights/reuse summaries remain.
