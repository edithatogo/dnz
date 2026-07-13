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

Required secrets are `HF_TOKEN` and `ZENODO_TOKEN`. DigitalNZ public content is discovered without a key; an optional `DIGITALNZ_API_KEY` can be supplied only for an approved higher-throughput allocation. The Hugging Face token must be fine-grained to the public dataset and must have accepted access to `pyannote/speaker-diarization-community-1`.

Required repository variables are `HF_REPO_ID=edithatogo/digitalnz`, `GH_ZERO_COST_REVIEWED_AT=YYYY-MM-DD`, and `HF_ZERO_COST_REVIEWED_AT=YYYY-MM-DD`. Reviews expire after 90 days. Only after account review and the pilot checkpoint should `RNZ_ARCHIVE_ENABLED=true` be set.

For a personal GitHub repository, the current budget REST API cannot create an Actions budget at personal-account scope. Confirm the account-level Actions spending limit in GitHub billing, keep this repository public, and record the review date. The workflow still rejects every paid runner label regardless of the account setting.

Hugging Face billing review must confirm there are no RNZ-related Jobs, endpoints, paid Spaces hardware, storage add-ons, or organization billing. Publication uses only the fixed public dataset and stops on free-capacity failure.

The local `hf` CLI is installed for account inspection but intentionally remains signed out unless an operator authenticates it with a fine-grained dataset token. GitHub Actions uses the existing repository secret without exposing it locally.

The `edithatogo` account accepted the contact-sharing conditions for `pyannote/speaker-diarization-community-1` on 2026-07-13, and Actions run `29244743661` proved that the repository token can read the gated model.

## Outputs

Each item records source and normalized audio, transcript JSON, anonymous speaker labels, RTTM, SRT, WebVTT, text, checksums, quality flags, rights basis, and exact model versions. It also includes `analysis.json` and `chapters.json` derivatives with speech coverage, word confidence, repetition, overlap, anonymous speaker count, pause-based chapters, broadcast-section hints, Māori-review signals, and conservative public-policy topic hints. Monthly WebDataset and Parquet shards are mirrored to Hugging Face and released through Zenodo when changed content exists.

The analysis fields are search and review aids. They are not editorial classifications, definitive language identification, speaker identification, or evidence about a person's identity. Canonical transcript text remains separately preserved and is never overwritten by enrichment.

## Follow-on enrichment

Conductor tracks 23 through 26 govern acoustic event and music segmentation, Māori and multilingual evaluation, entity-linked search, compound-page extraction, perceptual deduplication, post-publication integrity audits, and sensitive-content review. These functions require measured evaluation before they can run automatically on production recordings.

Machine transcripts and diarization are research artifacts and may contain errors. Speaker labels are anonymous and do not identify people.
