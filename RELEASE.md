# Release policy and checklist

## Compatibility policy

- The documented MSRV is Rust 1.75, matching the Pixi environment contract. A release must pass the MSRV check in CI or record a deliberate policy update here before publication.
- Public Rust, Python, CLI, and MCP behavior follows SemVer. Breaking changes require a major version; compatible additions and bug fixes use minor/patch releases as appropriate.
- Deprecations remain documented for at least one minor release, include a replacement, and are removed only at a planned major release.
- Provider contract changes are recorded in `conductor/contracts/` with fixture-backed tests and explicit live-validation status.

## Release checklist

- [ ] Update versions and `CHANGELOG.md`.
- [ ] Run format, workspace Clippy, workspace tests, Python/MCP/docs/package checks, audit/license, and secret/SBOM gates.
- [ ] Run `cargo bench -p dnz-core --bench benchmarks` and inspect regressions.
- [ ] Run the OpenAPI drift report; run live smoke only with explicit opt-in and redacted output.
- [ ] Build CLI/MCP binaries on Linux, Windows, and macOS; build and metadata-check the Python wheel and Conda recipe.
- [ ] Stage each archive with `LICENSE`, `README.md`, and `CHANGELOG.md`; run `scripts/verify-release-archives.py`.
- [ ] Verify `SHA256SUMS`, release notes, and that archives contain no source submodules, credentials, or build directories.
- [ ] Publish only after hosted CI and release assets are independently inspectable.
