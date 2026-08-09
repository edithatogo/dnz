# Python 3.14-only migration baseline

Date: 2026-08-10 (Australia/Sydney)

## Scope inventory

- `pixi.toml`: Python `>=3.12,<3.13`.
- `crates/dnz-python/pyproject.toml`: Python `>=3.8`.
- Seven GitHub workflow jobs declare Python 3.12.
- `scripts/verify-python-wheel.ps1` retains a `Python312` fallback.
- `rnz/transcription-requirements.lock` records compilation for Python 3.12.

## Red policy gate

Command:

```text
uv run --python 3.14 --no-project python -m unittest tests.test_python_runtime_policy -v
```

Result: expected failure. The policy validator reported 12 declarations that do not satisfy the approved Python 3.14-only policy.
