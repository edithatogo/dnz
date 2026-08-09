# Review: Python 3.14-only runtime

Date: 2026-08-10
Range: `origin/main...503774f`
Outcome: Passed after fixes

## Requirements review

- All supported runtime declarations resolve to Python 3.14.
- Pixi and RNZ locks install under CPython 3.14 with strict lock/hash checks.
- Binding wheel build, metadata validation, install, and import pass under CPython 3.14.
- Rust formatting, Clippy, workspace tests, audit, repository validation, hosted coverage, documentation, dependency review, Python analysis, and CodeQL pass.
- The dirty primary checkout, credential boundaries, and live-publication boundaries were preserved.

## Finding and resolution

One correctness finding was identified: equal speaker-word counts used a set during segment-label selection, making tie-breaking nondeterministic. Commit `503774f` preserves first-observed speaker order and adds a regression test. The targeted locked-environment tests and every hosted check passed after the fix.

## Remaining findings

None.
