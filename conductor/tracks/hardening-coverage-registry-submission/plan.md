# Track Plan: Hardening, Coverage & Registry Submission

- [~] Task 13.1: Configure GitHub Actions secrets mapping in `.github/workflows/release.yml`.
  - *Evidence:* Local workflow edit maps `CARGO_REGISTRY_TOKEN` and `PYPI_API_TOKEN` and keeps registry actions to dry-run/build/check steps only; validation is still blocked by the local linker/target access environment.
  - *Commit:* `chore(track-13): task 13.1 - configure release workflow secrets mapping`
- [~] Task 13.2: Verify line coverage exceeds 90% target locally using `cargo tarpaulin`.
  - *Evidence:* Added additional pure/unit coverage for `client`, `dataframe`, `digest`, `export`, `autopilot`, CLI parsing/diagnostics, MCP JSON-RPC request handling, and property tests, including replacing the previously empty property-test assertion. Coverage measurement remains blocked because cargo cannot complete host build-script linking in this local environment.
  - *Latest coverage increment:* `dnz-mcp` now has hermetic tests for tool schema discovery, `initialize`, error cases, mocked `search_digitalnz`, and mocked `get_digitalnz_facets`; `dnz-cli` now covers bbox parsing plus search/facets/doctor Clap decoding.
  - *Latest validation:* `cargo fmt --all`, `cargo metadata --no-deps --format-version 1`, and `git diff --check` pass after the MCP/CLI coverage increment.
  - *Latest blocker evidence:* Escalated `cargo check -p dnz-core --lib` can write to `C:\tmp`, but fails before project compilation because `link.exe` resolves to Git's POSIX linker; forcing Rust's bundled `rust-lld.exe` then fails because Windows SDK libraries such as `kernel32.lib` are not discoverable.
  - *Commit:* `test(track-13): task 13.2 - verify code coverage exceeds 90 percent`
- [~] Task 13.3: Profile and optimize Hot-paths in vectors similarity search.
  - *Evidence:* `cosine_similarity` now uses a fused iterator fold and `MemoryVectorStore::query_similarity` uses bounded `select_nth_unstable_by` partial selection before sorting only the top-k window. Unit coverage covers identical, orthogonal, mismatched, empty, insert/get, top-k, empty-store, and limit-greater-than-count cases.
  - *Validation:* `cargo fmt --all`, `cargo metadata --no-deps --format-version 1`, and `git diff --check` pass; benchmark execution remains blocked by the local Windows linker/SDK environment.
  - *Commit:* `perf(track-13): task 13.3 - profile and optimize vector search hot-paths`
- [~] Task 13.4: Complete dry-run publish checks for Maturin and Cargo workspaces.
  - *Evidence:* `pixi.toml` defines `dry-run-cargo` and `dry-run-maturin`; `.github/workflows/release.yml` includes crates.io dry-run jobs for `dnz-core`, `dnz-cli`, and `dnz-mcp`, plus a maturin wheel build and twine metadata check for `dnz-python`. Registry token checks are read-only and do not publish.
  - *Packaging metadata:* Added MIT license metadata, repository/homepage/readme metadata, explicit `dnz-core = { version = "0.1.0", path = "../dnz-core" }` internal dependency declarations, Python project URL/license metadata, and conda `license_file` metadata.
  - *Validation:* Added `scripts/validate-package-metadata.ps1`, wired it into `pixi package-metadata` and CI, and verified it passes locally with `-AllowDirty`. The validator runs Cargo metadata checks, package-list checks for `dnz-core`, `dnz-cli`, and `dnz-mcp`, plus Python/conda license URL checks. Full local dry-runs remain blocked before build/package verification by the same target write/linker/Windows SDK issues.
  - *Commit:* `chore(track-13): task 13.4 - run package dry-runs for cargo and maturin`
