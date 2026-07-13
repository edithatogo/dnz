# Zero-Cost Account Audit

Audit date: 2026-07-13

## GitHub

- Account/repository: `edithatogo/dnz`
- Repository visibility: public.
- Custom/self-hosted runners: none registered.
- RNZ workflow runner: `ubuntu-latest` only.
- Payment method: absent. GitHub refuses creation of a budget until a payment method is added; none was added because doing so would weaken the zero-cost boundary.
- Repository environment: `rnz-zero-cost-production` created.
- Variables: fixed Hugging Face destination and dated zero-cost reviews configured; `RNZ_ARCHIVE_ENABLED=false`.
- Secrets present: `HF_TOKEN` and `ZENODO_TOKEN`; secret values were not read or exposed.

## Hugging Face

- Account: `edithatogo`.
- Subscription: free; the billing page offers PRO rather than showing an active subscription.
- Credits: `$0.00`.
- Automatic recharge: none.
- Current-period usage: `$0.00`.
- Compute and private-storage usage: none.
- Public storage: approximately `0.02 TB / 8.7 TB` at audit time.
- Local CLI: `hf` 1.4.1 installed; deliberately not authenticated with a broad personal token.
- Dataset: `edithatogo/digitalnz`, required to remain public.
- Diarization model: `pyannote/speaker-diarization-community-1` is gated and access has not yet been accepted. Production must remain disabled until the user authorizes the required contact-information disclosure and the token used by Actions is verified to have access.

## Activation Decision

Account-level zero-cost conditions pass, but the pilot is blocked on gated pyannote model access. Schedules are installed but safely suspended by `RNZ_ARCHIVE_ENABLED=false`.
