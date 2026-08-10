# T03 endpoint and model slice — 2026-07-14

Implemented and tested:

- `Client::record()` with path-safe `/records/{record_id}.json` construction, field selection, auth-header reuse, and structured transport/HTTP/decode errors.
- `Client::more_like_this()` with pagination, fields, nested filters, path-safe `/records/{record_id}/more_like_this.json` construction, and flat/enveloped response normalization.
- Tolerant record IDs accepting verified string and integer JSON forms.
- Typed common metadata fields including rights/use, URLs, dates, partner/collection, subject, language, location, and thumbnail metadata.
- Flattened unknown provider fields retained in `Record::extra_fields`.
- Search normalization for the existing envelope and flat `records`/`results` response shapes.
- Optional page/per-page, facets, and request metadata are preserved for both envelope and flat shapes.
- Verified XML support now parses the live v3 `<search>` shape, hyphenated field names, typed pagination, repeated values, direct record responses, and path-safe search/record builders.
- Added RSS 2.0 `<channel><item>` normalization with GUID/link/date/category mappings, unknown-field preservation, and fixture-backed model/client tests.

Verification:

```powershell
cargo +stable-x86_64-pc-windows-gnu test -p dnz-core --all-features
```

Pass: 68 unit tests, 9 client integration tests, 5 property tests, and 0 doctests failed.

Workspace verification also passed with `cargo fmt --all -- --check`,
`cargo metadata --no-deps --format-version 1`, `git diff --check`, and
`cargo +stable-x86_64-pc-windows-gnu clippy --workspace --all-targets
--all-features -- -D warnings` using the repository `.pixi` Python interpreter.

The official v3 XML endpoint was queried without credentials on 2026-07-14 and
returned the fixture shape covered by the tests. The corresponding RSS endpoint
returned HTTP 500. The repository now has a stable fixture-backed parser and
client path; the live HTTP 500 remains an external DigitalNZ service issue.

The expanded client integration coverage verifies structured mapping for HTTP
400, 403, 404, 429, 500, 502, and 503 responses; bounded `Retry-After`; stable
malformed-JSON decode errors; MLT status mapping; and secret-safe error text.

Remaining T03 work is limited to rechecking the live RSS endpoint when DigitalNZ restores it. JSON, XML, and fixture-backed RSS coverage is complete for the current builders.
