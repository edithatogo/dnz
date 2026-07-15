# T05 clean-room pydnz compatibility slice — 2026-07-14

Implemented:

- Added the installable `dnz.api` Python facade over the native Rust extension.
- Added clean-room `Dnz`, `Request`, and `Results` compatibility objects.
- Preserved `result_count`, `records`, `facets`, `errors`, `request`, and `raw` attributes.
- Added safe parameter construction, HTTPS request descriptions, credential-redacted representations, validated `extra_params`, and rejection of unsafe `wild`/credential keys.
- Added documented compatibility notes without copying GPL implementation, comments, or tests.
- Built and imported the CPython 3.12 wheel from `dist/dnz-0.1.0-cp312-cp312-win_amd64.whl`.

Verification:

```powershell
rustup run stable-x86_64-pc-windows-gnu cargo check -p dnz-python --all-features
.pixi/envs/default/python.exe -m compileall -q crates/dnz-python/python
```

Pass: native extension check completed without diagnostics, 3 compatibility tests passed, and isolated wheel extraction/import tests passed for both `dnz.api` and `dnz._native`.

Remaining T05 work: broader behavioral coverage for every documented filter combination; the required wheel-level import gate is complete.

Packaging remediation: `maturin` is now declared in `pixi.toml` and locked for
all platforms. The packaging task routes Cargo through the validated GNU
toolchain and isolated target directory; the old default-MSVC route is no
no longer used. The Python crate now disables the optional core dataframe
feature, keeping Polars out of the thin adapter wheel while preserving the
core crate's dataframe default for native consumers. The final wheel artifact
is now limited to the GNU native build completing in the local runner. The GNU build completed successfully in 6m 37s.
