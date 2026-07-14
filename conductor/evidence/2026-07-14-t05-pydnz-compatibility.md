# T05 clean-room pydnz compatibility slice — 2026-07-14

Implemented:

- Added the installable `dnz.api` Python facade over the native Rust extension.
- Added clean-room `Dnz`, `Request`, and `Results` compatibility objects.
- Preserved `result_count`, `records`, `facets`, `errors`, `request`, and `raw` attributes.
- Added safe parameter construction, HTTPS request descriptions, credential-redacted representations, validated `extra_params`, and rejection of unsafe `wild`/credential keys.
- Added documented compatibility notes without copying GPL implementation, comments, or tests.

Verification:

```powershell
rustup run stable-x86_64-pc-windows-gnu cargo check -p dnz-python --all-features
.pixi/envs/default/python.exe -m compileall -q crates/dnz-python/python
```

Pass: native extension check completed without diagnostics and Python facade compilation, unit, and smoke checks passed.

Remaining T05 work: broaden behavioral tests for all documented filter and pagination combinations and produce a wheel-level import test.

Packaging remediation: `maturin` is now declared in `pixi.toml` and locked for
all platforms. The packaging task routes Cargo through the validated GNU
toolchain and isolated target directory; the old default-MSVC route is no
longer used. A final wheel artifact still requires the long-running GNU build
to complete in the local runner.
