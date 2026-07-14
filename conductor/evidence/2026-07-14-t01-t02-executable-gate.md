# T01/T02 executable gate — 2026-07-14

## Environment blocker addressed

- Removed only known generated DNZ Cargo target directories from prior failed runs.
- Free space increased from approximately 3.1 GB to 50.39 GB.
- `scripts/workspace-doctor.ps1` confirms the supported GNU Rust route, MinGW GCC, writable external target, and valid learning-log schema.
- The default MSVC route remains unsuitable because `link.exe` resolves to Git's POSIX linker; the repository GNU fallback bypasses it.

## Verification

Command:

```powershell
$env:PATH = "$env:USERPROFILE\scoop\apps\mingw\current\bin;$env:PATH"
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'dnz-target-blocker'
$env:CARGO_BUILD_JOBS = '1'
$env:PYO3_PYTHON = Join-Path (Get-Location) '.pixi\envs\default\python.exe'
cargo +stable-x86_64-pc-windows-gnu test -p dnz-core --all-features
```

Result: pass — 57 unit tests, 4 client integration tests, 5 property tests, and 0 doctests failed.

## Fixes verified

- Nested boolean filter serialization now emits correctly bracketed paths such as `and[or][category][]`.
- The integration fixture now asserts the documented 60-second cap for an excessive `Retry-After` value.
- The property test now preserves the documented behavior that distinct ID-bearing records with empty titles remain distinct.
