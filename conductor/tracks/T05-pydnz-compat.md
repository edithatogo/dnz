# T05 — Clean-room pydnz compatibility

Implement `dnz.api.Dnz`, `Request`, and `Results` from public behaviour without copying GPL code. Cover documented search kwargs and the observed `_without` behaviour, preserve raw dictionaries, redact request representations, replace `wild` with safe `extra_params`, and document intentional modern differences.


## Completion record

Status: not_started

Evidence: none

Open decisions/blockers: wheel-level import validation is externally blocked because the configured Pixi environment has no `maturin` executable; broader behavioral coverage remains.
