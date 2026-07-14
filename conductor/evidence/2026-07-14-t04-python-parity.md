# T04 Python parity slice — 2026-07-14

Implemented:

- Added Python builders for `record()` and `more_like_this()` over `dnz-core`.
- Added field selection, pagination, and `send()`/`send_raw()` normalized JSON result methods.
- Registered both builders in the PyO3 module and documented the usage alongside CLI parity.

Verification:

```powershell
rustup run stable-x86_64-pc-windows-gnu cargo check -p dnz-python --all-features
rustup run stable-x86_64-pc-windows-gnu cargo clippy -p dnz-python --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

Pass: Python adapter compilation and Clippy with warnings denied; formatting check passed.

Remaining T04 work is limited to richer typed Python facade objects and broader public API documentation.
