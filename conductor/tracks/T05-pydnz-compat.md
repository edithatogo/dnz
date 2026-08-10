# T05 — Clean-room pydnz compatibility

Implement `dnz.api.Dnz`, `Request`, and `Results` from public behaviour without copying GPL code. Cover documented search kwargs and the observed `_without` behaviour, preserve raw dictionaries, redact request representations, replace `wild` with safe `extra_params`, and document intentional modern differences.


## Completion record

Status: complete

Evidence: conductor/evidence/2026-07-14-t05-pydnz-compatibility.md

Open decisions/blockers: None for the accepted track surface. Broader optional behavioral combinations remain future hardening.
