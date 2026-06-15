# Findings & Scratchpad

## Workspace Discoveries
- **Workspace root:** `dnz` configured as a multi-crate Cargo workspace.
- **Member Crates:**
  - `dnz-core`: Native library mapping client endpoints and data handling.
  - `dnz-cli`: Command Line Interface (`dnz` command).
  - `dnz-mcp`: Model Context Protocol server.
- **Environments:** Managed via `pixi` for cross-platform system libraries (openssl, cross-compilers).
- **Core target:** Port DigitalNZ API client to Rust using `reqwest` and `serde`.

## Advanced Features Reference
- **Semantic Vector:** Embeddings generated using `candle` offline model loader.
- **Query Autopilot:** Dynamic query splitting using facet metrics to fetch complete deep datasets.
- **FFI:** Python FFI bindings using `pyo3` and `maturin`.

## 2026-06-15 Swarm Reconciliation Notes
- Latest run inspected: `.swarm/runs/20260615-182124/manifest.json`.
- `general_coder.out.log` reports Tracks 1-12 complete, then self-selects Track 13 and starts release workflow/package edits.
- `quality_validator.out.log` reports critical issue C001: `rand::random::<u64>()` is used in `crates/dnz-core/src/client.rs`, but `rand` was missing from crate dependencies before the current uncommitted fix.
- Current dirty files after reconciliation: `.github/workflows/release.yml`, `Cargo.toml`, `crates/dnz-core/Cargo.toml`, and `pixi.toml`.
- Release workflow must remain non-uploading under the current swarm constraints: crate steps use `cargo publish --dry-run`, and the Python lane builds/checks wheels without running `maturin publish`.
- `crates/dnz-core/src/vector.rs` had a Cline hot-path optimization that selected nth index `k` even when `k == scores.len()`; reconciled by skipping `select_nth_unstable_by` when the caller asks for all scores.
- `cargo check` evidence: failed before local crate validation with access denied writing `.rmeta` files under `target\debug\deps`; linker lookup resolves `link.exe` to Git's `usr\bin\link.exe`, and no Visual Studio linker was found by the lightweight probe.
- `git status` evidence: normal status warns on stale `digitalnz/.git/index.lock` and `pydnz/.git/index.lock`; `git status --short --ignore-submodules=all` is the clean parent-repo status surface for now.
- Track 13 continuation added coverage-focused tests in `client.rs`, `dataframe.rs`, `digest.rs`, `export.rs`, and `property_tests.rs`; `cargo fmt --all`, `cargo metadata --no-deps --format-version 1`, `scripts/validate-tmdl.ps1`, and `git diff --check` pass.
- Added hermetic `Autopilot::harvest_deep` tests with `wiremock` for empty year facets and partitioned year harvesting with duplicate record ID reconciliation.
- Added MCP handler coverage in `crates/dnz-mcp/src/main.rs`: tool schema, `initialize`, error branches, mocked search, and mocked facet calls. Added `wiremock` as a `dnz-mcp` dev dependency.
- Added CLI parsing coverage in `crates/dnz-cli/tests/cli_tests.rs`: bbox parser success/failure, search option decoding, facets field splitting, and doctor subcommand parsing.
- Post MCP/CLI coverage validation evidence: `cargo fmt --all`, `cargo metadata --no-deps --format-version 1`, and `git diff --check` pass.
- Packaging dry-run blocker reduced: publishable Rust crate manifests now include license/repository/homepage/readme metadata, and dependent crates use versioned `dnz-core` path dependencies so crates.io packaging can resolve internal workspace dependencies. Python/conda metadata now carries matching MIT/license-file metadata.
- Package-shape validation evidence: `cargo package -p dnz-core --allow-dirty --list`, `cargo package -p dnz-cli --allow-dirty --list`, and `cargo package -p dnz-mcp --allow-dirty --list` pass without invoking local linker verification.
- Added repeatable package metadata validation in `scripts/validate-package-metadata.ps1`; `pixi.toml` exposes it as `package-metadata`, and CI runs it after TMDL validation. Local evidence: `powershell -NoProfile -ExecutionPolicy Bypass -File scripts\validate-package-metadata.ps1 -AllowDirty` passes.
- `cargo check -p dnz-core --lib` with `CARGO_TARGET_DIR=C:\tmp\dnz-target` still fails before project compilation with `Access is denied` while creating a target temp path, so coverage and package dry-runs remain environment-blocked rather than code-validated.
- Escalated `cargo check -p dnz-core --lib` proves target writes can proceed outside the sandbox, but compilation still fails before crate code because PATH resolves `link.exe` to `C:\Users\60217257\scoop\apps\git\current\usr\bin\link.exe`. Forcing Rust's bundled `rust-lld.exe` avoids Git `link.exe` but fails on missing Windows SDK libs (`kernel32.lib`, `ntdll.lib`, `userenv.lib`, `ws2_32.lib`, `dbghelp.lib`). `scripts/workspace-doctor.ps1` now reports these separately.
