# Python 3.14 migration validation

Date: 2026-08-10

## Runtime and dependency evidence

- `pixi lock` and `pixi install --locked` completed with Python 3.14.6.
- The hash-locked RNZ environment installed under CPython 3.14.6 with `uv pip install --require-hashes --index-strategy unsafe-best-match`.
- Imports succeeded for Torch 2.10.0+cpu, Torchvision 0.25.0+cpu, Torchaudio 2.10.0+cpu, Faster Whisper 1.2.1, pyannote.audio 4.0.7, and PyArrow 23.0.1.
- The full Python test discovery passed: 44 tests, zero failures, and zero skips when run in the RNZ environment.

## Repository gates

- `pixi run verify-local` passed. This covered Rust formatting, Clippy with warnings denied, all Rust tests, RustSec audit, strict workspace doctor, TMDL validation, and package metadata.
- The CPython 3.14 Windows wheel built through the GNU Rust toolchain as `dnz-0.1.0-cp314-cp314-win_amd64.whl`.
- `twine check` passed and the built wheel imported successfully under CPython 3.14.
- `pixi run coverage` stopped at its explicit Windows host prerequisite gate because the GNU toolchain lacks `profiler_builtins` and the host has no usable Visual Studio linker/SDK route. Coverage remains a required hosted-CI gate; it was not counted as a local pass.

## Compatibility correction

WhisperX does not support Python 3.14 and pins an older Torch family. The RNZ pipeline now uses its already-pinned underlying Faster Whisper and pyannote libraries directly. Unit tests cover canonical segment conversion, pyannote 4 output conversion, anonymous speaker assignment, unknown-speaker fallback, and the full packaging path.

## Hosted evidence

PR #43 completed every required check successfully on commit `7f21b14`:

- Python analysis: <https://github.com/edithatogo/dnz/actions/runs/31321243353/job/93264461151>
- Dependency review: <https://github.com/edithatogo/dnz/actions/runs/31321243346/job/93264461119>
- Astro documentation: <https://github.com/edithatogo/dnz/actions/runs/31321243349/job/93264461130>
- Code quality and test coverage: <https://github.com/edithatogo/dnz/actions/runs/31321243366/job/93264461226>
- CodeQL: <https://github.com/edithatogo/dnz/runs/93264579735>

The post-review fix on commit `503774f` also passed the complete hosted set, including quality and coverage at <https://github.com/edithatogo/dnz/actions/runs/31321603626/job/93265356218>.
