# Track Specification: Zero-Cost Continuous RNZ Audio Archive

## Overview

Build a resumable GitHub Actions pipeline that discovers rights-cleared RNZ recordings through DigitalNZ, downloads bounded batches, creates CPU-based transcripts and anonymous speaker diarization, and publishes audio plus derivatives to public Hugging Face and monthly Zenodo archives without enabling paid compute or storage.

## Functional Requirements

- Run daily and by manual dispatch on standard `ubuntu-latest` runners only.
- Fail closed when the repository is private, zero-cost policy fails, required rights evidence is absent, or free archive capacity is unavailable.
- Maintain an append-only item manifest with provenance, checksums, state transitions, retries, models, and archive locations.
- Allow only HTTPS media from configured RNZ domains, with redirect, size, duration, type, and checksum validation.
- Transcribe with pinned faster-whisper models, align supported languages with WhisperX, and diarize anonymously with pinned pyannote Community-1.
- Publish WebDataset audio shards, Parquet metadata/transcripts, captions, RTTM, checksums, and provenance to a public Hugging Face dataset.
- Publish changed-content-only monthly Zenodo versions below the free record limits.
- Pause and open a durable issue when any step would require paid compute, paid storage, or unverified billing state.

## Non-Functional Requirements

- Never invoke Hugging Face Jobs, endpoints, paid runners, paid Spaces hardware, storage add-ons, or organization billing.
- Never leak DigitalNZ, Hugging Face, pyannote, or Zenodo credentials.
- Keep each scheduled run resumable and below GitHub's six-hour execution limit.
- Pin action revisions and Python/model dependencies.
- Make high and critical security findings block processing and publication.

## Acceptance Criteria

- Zero-cost policy checks reject paid runner and Hugging Face compute configurations.
- Discovery, validation, state transitions, retries, and archive assembly have offline tests.
- Production workflows never fetch RNZ media during pull-request validation.
- A 100-item pilot mode and bounded historical backfill mode are available.
- Workflow summaries report progress, failures, free-capacity state, and model provenance.
- Account audit documents GitHub visibility/budget and Hugging Face billing/resource state without claiming unverifiable settings.

## Out of Scope

- Speaker identification or voiceprinting.
- Paid hosted GPU, CPU, endpoint, runner, storage, or inference services.
- Automatic rights inference from an `All rights reserved` metadata label.
- Guaranteeing a completion date for the full historical corpus.
