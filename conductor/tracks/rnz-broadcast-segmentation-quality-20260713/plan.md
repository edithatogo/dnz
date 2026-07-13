# Track Plan: RNZ Broadcast Segmentation & Quality

- [ ] Select and pin CPU acoustic segmentation components and evaluation fixtures that complete within the standard free-runner budget.
- [~] Implement audio-quality, speech/music and acoustic-event derivatives.
  - [x] Add speech coverage, confidence, repetition, overlap and possible non-speech review metrics.
  - [x] Add streaming deterministic RMS, clipping and near-silence metrics using FFmpeg PCM.
  - [ ] Add evaluated zero-cost acoustic speech/music/event models; defer models that exceed the runner budget.
- [~] Implement broadcast sectioning, chapter reconciliation and confidence thresholds.
  - [x] Add deterministic section hints and pause-based chapters with explicit limitations.
  - [ ] Evaluate boundaries and reconcile chapters against programme metadata.
- [ ] Evaluate on the reviewed pilot, apply fixes and document limitations.
- [ ] Run Conductor review and archive the track.
