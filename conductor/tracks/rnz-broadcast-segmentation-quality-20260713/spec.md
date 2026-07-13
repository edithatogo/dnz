# Track Specification: RNZ Broadcast Segmentation & Quality

## Requirements

- Detect speech, music, silence, applause, laughter, telephone-band audio and overlapping speech with pinned CPU-capable components.
- Produce chapter boundaries and typed programme sections while preserving offsets into the canonical recording.
- Measure clipping, loudness, timestamp drift, speech coverage, repetition and transcription confidence.
- Route uncertain output to review; never classify music lyrics as reliable speech automatically.
- Benchmark against synthetic fixtures and a manually reviewed stratified RNZ sample without production downloads in pull requests.
- Preserve originals and record every restoration or denoising operation as an optional derivative.

## Acceptance Criteria

- Acoustic labels and chapters are machine-readable, timestamped and independently disableable.
- False-positive and false-negative measurements are documented for each supported label.
- All processing uses standard public GitHub runners and passes the zero-cost policy.
