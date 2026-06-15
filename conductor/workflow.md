# Development Workflow: DigitalNZ Integration Hub (Rust Core)

## 1. Branching & Commit Protocol

### Task-Level Commits (Micro-commits)
- Developers must commit after completing every single task in a track plan.
- Commit messages must follow the Conventional Commits format (e.g. `feat(cli): implement json output format` or `test(mcp): add property-based tests for pagination`).

### Phase-Level Verification
- A phase constitutes a logical grouping of tasks.
- Prior to pushing to the remote repository after a phase:
  1. Run `cargo fmt --check`.
  2. Run `cargo clippy --all-targets -- -D warnings`.
  3. Run `cargo test`.
  4. Perform a local peer-review of diffs.
- Once verified, push the phase branch and raise/update the Pull Request.

### Track-Level Release & CI Validation
- Upon completing all tasks in a track:
  1. Execute the full test suite including property-based testing and mutation testing (`cargo mutants`).
  2. Review coverage reports to ensure total coverage is >90%.
  3. Push to `main` (or merge the PR) to trigger the GitHub Actions workflow.
  4. Verify that the GitHub Actions run completes successfully, checking binary cross-compilation and automated release tests.

## 2. Release & Versioning Strategy

### Semantic Versioning (SemVer)
- Release versions follow standard SemVer 2.0.0.
- Manifest versions are updated in `Cargo.toml` before tagging release commits.
- Git releases are created via tags matching `vX.Y.Z`.

### Continuous Deployment (CD) Automation
- On pushing a tag `vX.Y.Z`:
  - GitHub Actions compiles the Rust project for Windows, macOS, and Linux targets in release mode using `cargo-dist`.
  - Releases are created on GitHub with the compiled binaries attached.
  - Automatically exports target schemas and publishes registry configuration updates for the MCP registries and IDE marketplace registries.
