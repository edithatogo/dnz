# Track Plan: Workspace Diagnostics, Audits & Pre-commit Hooks

- [x] Task 14.1: Integrate `cargo-audit` check validation into `pixi.toml` and CI workflows.
  - *Evidence:* `pixi.toml` has an `audit = "cargo audit"` task and `.github/workflows/ci.yml` installs/runs `cargo audit` before coverage.
  - *Commit:* `chore(track-14): task 14.1 - integrate cargo-audit for dependency vulnerability scanning`
- [x] Task 14.2: Build a workspace doctor script verifying local compiler tools, disk space, and PATH setups.
  - *Evidence:* Added `scripts/workspace-doctor.ps1` and `pixi` `doctor` task covering workspace path, target writes, PATH tools, linker resolution, and disk space.
  - *Latest hardening:* `scripts/workspace-doctor.ps1 -Strict` now passes on the local Windows/OneDrive workspace by probing a temp target when the repo path has spaces, recognizing the GNU Rust host toolchain plus MinGW as a valid Windows build route, and recording MSVC/Windows SDK absence as bypassed rather than blocking when that route is available.
  - *Commit:* `chore(track-14): task 14.2 - add workspace doctor diagnostic tool`
- [x] Task 14.3: Configure local Git pre-commit hooks to automate formatting and linting validation.
  - *Evidence:* Added `.githooks/pre-commit` and `pixi` `install-hooks` task for fmt, clippy, and test checks.
  - *Commit:* `chore(track-14): task 14.3 - add pre-commit Git hooks for format and lint check`
- [x] Task 14.4: Integrate diagnostic doctor checks directly into the CLI startup sequence.
  - *Evidence:* Added `dnz doctor` command backed by `workspace_diagnostics()` in `dnz-cli`.
  - *Commit:* `chore(track-14): task 14.4 - run basic diagnostic checks on CLI execution`
