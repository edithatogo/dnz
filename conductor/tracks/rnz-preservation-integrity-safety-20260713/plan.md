# Track Plan: RNZ Preservation Integrity & Sensitive-Content Safety

- [x] Implement parent/child recording discovery and archive manifest fields.
- [~] Implement perceptual fingerprints and duplicate relationship classification.
  - [x] Add normalized-audio exact duplicate relationships.
  - [ ] Add evaluated perceptual, excerpt and rebroadcast relationships.
- [~] Implement post-publication decodability, checksum and offset audits.
  - [x] Verify local derivatives, duration preservation, checksums and anonymous labels before packaging.
  - [x] Download each newly published Hugging Face shard and verify its complete checksum manifest.
  - [x] Verify Zenodo server checksums for every file and download each checksum manifest after publication.
- [~] Implement conservative sensitive-content review signals and operator workflow.
  - [x] Add review-only signals with a no-automatic-restriction contract.
  - [ ] Add operator review state, dispositions and audit history.
- [ ] Evaluate, review and archive the track.
