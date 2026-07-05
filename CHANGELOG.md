# Changelog

## v0.1.0 - 2026-07-05

Initial public release candidate for the DigitalNZ Integration Hub.

### Added
- Rust `dnz-core` client for DigitalNZ search, filters, facets, retries, and persistent SQLite response caching.
- `dnz-cli` command-line interface with search, facets, doctor, structured logging, cache path configuration, and New Zealand Gazette export mode.
- `dnz-mcp` stdio JSON-RPC MCP server exposing DigitalNZ search and facet tools.
- `dnz-python` PyO3 bindings for Python search workflows.
- Local export helpers for Frictionless Data Package, schema.org JSON-LD, Gazette JSONL, raw page JSON, and manifests.
- Local vector-search utilities and embedding model artifact download helpers.
- Astro/Starlight documentation, Power BI semantic model scaffold validation, packaging metadata validation, and Conductor track/archive context.

### Validation
- Main branch Rust CI/CD passes formatting, clippy, dependency audit, TMDL validation, package metadata validation, tests, and coverage.
- Main branch Docs workflow passes.
- Local `pixi run verify-local` passes on the Windows workstation using the GNU Rust build route.

### Known Follow-Ups
- Crates.io dry-run validation can be blocked by transient crates.io network slowness on this workstation.
- Local Windows Python wheel builds remain sensitive to Python/Rust linker selection; the release workflow builds Python packaging on Ubuntu.
