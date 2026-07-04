# Track Plan: Persistent Cache & Advanced API Features

- [x] Task 17.1: Integrate SQLite database schema configuration into `dnz-core`.
  - *Evidence:* Added `PersistentCache` with SQLite schema initialization, schema metadata, read/write/clear operations, and unit coverage.
  - *Commit:* `feat(track-17): task 17.1 - set up local SQLite cache schema`
- [x] Task 17.2: Implement persistent caching query checks in client middleware.
  - *Evidence:* `Client::with_cache_path` enables SQLite-backed cache reuse across client instances while retaining in-memory cache behavior and excluding API keys from cache keys.
  - *Commit:* `feat(track-17): task 17.2 - integrate SQLite persistent caching in client query builder`
- [x] Task 17.3: Implement automatic model downloader in the vector search module.
  - *Evidence:* Added `EmbeddingModelDownload` and `ensure_embedding_model`, with tests for missing-file download and existing-file reuse.
  - *Commit:* `feat(track-17): task 17.3 - implement automated model downloading for embeddings`
- [x] Task 17.4: Add structured JSON-LD logging formatting options.
  - *Evidence:* CLI supports `--log-format json`; MCP supports `DNZ_LOG_FORMAT=json`; both keep logging on stderr and preserve command/stdout protocols.
  - *Commit:* `feat(track-17): task 17.4 - add structured JSON logging options to CLI/MCP`
