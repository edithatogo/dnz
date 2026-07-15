# T07 bounded page stream evidence

## Implemented slice

- Added `Client::search_pages`, a lazy `SearchPageStream` over the existing normalized search request path.
- The stream fetches only when `next_page` is called, clamps `per_page` to the provider limit, supports a hard `max_pages` bound, and stops after an empty result page.
- Dropping the stream cancels future work because no background task or eager buffer is created.

## Verification

- `cargo fmt --all -- --check` — PASS.
- `rustup run stable-x86_64-pc-windows-gnu cargo test -p dnz-core --all-features` — PASS (69 unit, 11 integration, 5 property, 0 doctest failures).
- The integration test `bounded_search_page_stream_fetches_on_demand` verifies page ordering, request bounds, and terminal `None` behavior against wiremock.

## Cache lifecycle slice

- Persistent cache reads now accept an optional TTL and reject expired entries without deleting provenance-bearing cache rows.
- `Client::with_cache_ttl` makes freshness explicit, while `Client::offline` prevents network access and returns a stable error when no usable cached response exists.
- TTL-configured queries bypass the non-timestamped in-memory shortcut, so stale responses cannot be returned accidentally.

## Cache verification

- `persistent_cache_ttl_rejects_expired_entries` and `offline_mode_fails_without_a_cached_response` pass in the core gate.

## Cache bounds and provenance slice

- Migrated the SQLite cache schema to version 2, adding source URL and authentication namespace provenance while preserving existing v1 databases.
- Added deterministic oldest-first eviction through `PersistentCache::prune_to_limit` and client configuration through `with_cache_max_entries`.
- `persistent_cache_records_provenance_and_prunes_oldest_entries` verifies both behaviors.

## Latest verification

- `cargo fmt --all -- --check` — PASS.
- `rustup run stable-x86_64-pc-windows-gnu cargo test -p dnz-core --all-features` — PASS (72 unit, 11 integration, 5 property, 0 doctest failures).

## Remaining T07 work

Recursive harvest planning with checkpoint/resume/rate budgets and deterministic incremental-sync manifests remain open.
