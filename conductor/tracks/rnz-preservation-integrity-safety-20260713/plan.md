# Track Plan: RNZ Preservation Integrity & Sensitive-Content Safety

- [x] Implement parent/child recording discovery and archive manifest fields.
- [~] Implement perceptual fingerprints and duplicate relationship classification.
  - [x] Add normalized-audio exact duplicate relationships.
  - [ ] Add evaluated CPU-bounded perceptual, excerpt and rebroadcast relationships that fit free runners; defer corpus-wide matching to optional local compute.
- [~] Implement post-publication decodability, checksum and offset audits.
  - [x] Verify local derivatives, duration preservation, checksums and anonymous labels before packaging.
  - [x] Download each newly published Hugging Face shard and verify its complete checksum manifest.
  - [x] Verify Zenodo server checksums for every file and download each checksum manifest after publication.
- [~] Implement conservative sensitive-content review signals and operator workflow.
  - [x] Add review-only signals with a no-automatic-restriction contract.
  - [x] Add versioned pending/not-required review records, durable reasons and workflow summary counts.
  - [ ] Add authenticated operator dispositions and append-only review audit history.
- [ ] Evaluate, review and archive the track.
