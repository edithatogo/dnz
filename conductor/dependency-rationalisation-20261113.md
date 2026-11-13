# Rust dependency rationalisation — dnz (2026-11-13)

## Repository-specific decisions

- Verified: workspace already declares Tokio full, Serde, Reqwest 0.12, Tracing, Thiserror 1.x, Anyhow, Clap, Proptest, Polars-core and Arrow/Parquet.
- Audit feature duplication across client, CLI, MCP and Python adapter. Evaluate narrowed Tokio/Reqwest defaults and Thiserror 1→2 only with complete CLI/MCP and wire-format parity.
- Keep Arrow/Parquet as exchange contracts; Polars-core is justified only where transformations need it. SQLx is not an automatic replacement for existing rusqlite persistence.
- Extend Proptest for acquisition receipt, XML record identity, deduplication and path-normalization; Criterion on fixed API replay fixtures; Insta on small scrubbed public metadata schemas.
- PyO3 must remain confined to the Python binding crate. Rayon/Crossbeam only for independently benchmarked source-item processing preserving ordered provenance.

## Evidence and implementation gates

- Status: **planned / scope registered**; no runtime substitution or CI/benchmark success is asserted by this file.
- Reconcile existing Conductor tracks before new work; preserve domain-specific APIs, schema versions, privacy and public CLI behaviour.
- Write failing contract tests before code changes, then run native targeted/full tests, fmt/clippy and relevant integration/supply-chain checks at exact SHA.
- Use Cargo feature/duplicate inventories plus security, licence and MSRV assessments, and fixed-fixture performance baselines before adoption.
- Record adopt / retain / reject / defer decisions and rollback; only close after verified evidence.
