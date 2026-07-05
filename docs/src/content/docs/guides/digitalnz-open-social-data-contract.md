---
title: DigitalNZ and Open Social Data Contract
---

# DigitalNZ and Open Social Data Contract

`dnz` is the upstream DigitalNZ integration layer.
`open_social_data` is the downstream curated dataset layer.

## Boundary

`dnz` owns:

- Low-level DigitalNZ API access
- CLI search, facets, Gazette export, and diagnostics
- MCP server tooling and release artifacts
- Python bindings and Rust package surfaces
- Persistent cache configuration

`open_social_data` owns:

- Curated dataset provider registration
- Catalog and search workflows
- Quality reports and validation
- Parquet export and dataset normalization

## Stable Surfaces

The following `dnz` surfaces are intended for downstream reuse:

- `dnz-core` client, models, query builder, and cache hooks
- CLI JSON output for `search`, `facets`, and `gazette-export`
- Gazette export manifest and JSONL record layout
- `DIGITALNZ_API_KEY` and `--api-key` credential handling

The following remain `dnz`-only:

- Interactive ad hoc search and facet exploration
- MCP server use as a direct user-facing tool
- Python notebook workflows
- `dnz doctor`

## Recommended Integration Mode

For the first `open_social_data` DigitalNZ provider:

- Use `dnz-core` directly for reusable request/response logic where it is stable.
- Treat Gazette export artifacts as the interchange path for repeatable dataset ingestion.
- Keep provider tests hermetic by using fixtures and mock responses.

## Compatibility Checklist

When changing `dnz`, check whether the change affects:

- CLI JSON structure
- Gazette export manifest fields
- JSONL record layout
- Cache path handling
- Credential handling
- Output determinism for fixtures and catalog updates

## Upstream Status

- `dnz` contract issue: [#3](https://github.com/edithatogo/dnz/issues/3)
- `dnz` live registry issue: [#4](https://github.com/edithatogo/dnz/issues/4)
- `open_social_data` provider track: [Track 13](https://github.com/edithatogo/open_social_data/issues/5)
