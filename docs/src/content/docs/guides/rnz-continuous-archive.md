---
title: RNZ Continuous Audio Archive
description: Zero-cost, rights-gated RNZ audio transcription, diarization, and archival workflow.
---

# RNZ Continuous Audio Archive

The RNZ archive pipeline uses standard Linux GitHub-hosted runners in this public repository. It does not use Hugging Face Jobs, endpoints, paid runners, paid Spaces hardware, storage buckets, or storage add-ons.

## Safety boundary

- `rnz/archive-policy.json` is the source of truth for rights authorization, collections, media domains, download limits, model revisions, and fixed archive namespace.
- Production processing remains suspended unless the repository variable `RNZ_ARCHIVE_ENABLED` is exactly `true`.
- The workflows fail if the repository is not public or the Hugging Face destination is not `edithatogo/digitalnz`.
- Free-capacity or billing uncertainty stops processing; it never enables paid capacity.

## Secrets and variables

Required secrets are `DIGITALNZ_API_KEY`, `HF_TOKEN`, and `ZENODO_TOKEN`. The Hugging Face token must be fine-grained to the public dataset and must have accepted access to `pyannote/speaker-diarization-community-1`.

Required repository variables are `HF_REPO_ID=edithatogo/digitalnz`, `GH_ZERO_COST_REVIEWED_AT=YYYY-MM-DD`, and `HF_ZERO_COST_REVIEWED_AT=YYYY-MM-DD`. Reviews expire after 90 days. Only after account review and the pilot checkpoint should `RNZ_ARCHIVE_ENABLED=true` be set.

For a personal GitHub repository, the current budget REST API cannot create an Actions budget at personal-account scope. Confirm the account-level Actions spending limit in GitHub billing, keep this repository public, and record the review date. The workflow still rejects every paid runner label regardless of the account setting.

Hugging Face billing review must confirm there are no RNZ-related Jobs, endpoints, paid Spaces hardware, storage add-ons, or organization billing. Publication uses only the fixed public dataset and stops on free-capacity failure.

## Outputs

Each item records source and normalized audio, transcript JSON, anonymous speaker labels, RTTM, SRT, WebVTT, text, checksums, quality flags, rights basis, and exact model versions. Monthly WebDataset and Parquet shards are mirrored to Hugging Face and released through Zenodo when changed content exists.

Machine transcripts and diarization are research artifacts and may contain errors. Speaker labels are anonymous and do not identify people.
