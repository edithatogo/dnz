# T07 bounded page stream evidence

## Implemented slice

- Added `Client::search_pages`, a lazy `SearchPageStream` over the existing normalized search request path.
- The stream fetches only when `next_page` is called, clamps `per_page` to the provider limit, supports a hard `max_pages` bound, and stops after an empty result page.
- Dropping the stream cancels future work because no background task or eager buffer is created.

## Verification

- `cargo fmt --all -- --check` — PASS.
- `rustup run stable-x86_64-pc-windows-gnu cargo test -p dnz-core --all-features` — PASS (69 unit, 11 integration, 5 property, 0 doctest failures).
- The integration test `bounded_search_page_stream_fetches_on_demand` verifies page ordering, request bounds, and terminal `None` behavior against wiremock.

## Remaining T07 work

Cache TTL/offline/eviction/provenance, recursive harvest planning with checkpoint/resume/rate budgets, and deterministic incremental-sync manifests remain open.
