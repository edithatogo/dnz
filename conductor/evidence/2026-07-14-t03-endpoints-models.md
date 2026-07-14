# T03 endpoint and model slice — 2026-07-14

Implemented and tested:

- `Client::record()` with path-safe `/records/{record_id}.json` construction, field selection, auth-header reuse, and structured transport/HTTP/decode errors.
- `Client::more_like_this()` with pagination, fields, nested filters, path-safe `/records/{record_id}/more_like_this.json` construction, and flat/enveloped response normalization.
- Tolerant record IDs accepting verified string and integer JSON forms.
- Typed common metadata fields including rights/use, URLs, dates, partner/collection, subject, language, location, and thumbnail metadata.
- Flattened unknown provider fields retained in `Record::extra_fields`.
- Search normalization for the existing envelope and flat `records`/`results` response shapes.

Verification:

```powershell
cargo +stable-x86_64-pc-windows-gnu test -p dnz-core --all-features
```

Pass: 60 unit tests, 8 client integration tests, 5 property tests, and 0 doctests failed.

The expanded client integration coverage verifies structured mapping for HTTP
400, 403, 404, 429, 500, 502, and 503 responses; bounded `Retry-After`; stable
malformed-JSON decode errors; MLT status mapping; and secret-safe error text.

Remaining T03 work is XML/RSS parsing only after verified fixtures are available, plus broader endpoint error-shape coverage.
