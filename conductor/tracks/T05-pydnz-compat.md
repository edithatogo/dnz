# T05 — Clean-room pydnz compatibility

Implement `dnz.api.Dnz`, `Request`, and `Results` from public behaviour without copying GPL code. Cover documented search kwargs and the observed `_without` behaviour, preserve raw dictionaries, redact request representations, replace `wild` with safe `extra_params`, and document intentional modern differences.


## Completion record

Status: not_started

Evidence: none

Open decisions/blockers: `maturin` is now provisioned and the task routes through GNU Rust; wheel-level artifact/import verification remains pending completion of the long-running local build. Broader behavioral coverage remains.
