# Track Specification: RNZ Māori & Multilingual Enrichment

## Requirements

- Evaluate segment-level language identification for te reo Māori, English and represented Pacific languages.
- Preserve macrons, code-switching, original model output and word/timestamp alignment.
- Use versioned, source-attributed dictionaries for names, iwi, hapū, places and institutions.
- Store corrections and translations only as reversible derivatives with confidence and provenance.
- Include Māori speech, names and code-switching in manual evaluation; do not infer ethnicity or identity.

## Acceptance Criteria

- Canonical output remains immutable and every correction links to original word offsets.
- Evaluation reports separate accuracy for Māori words/names and language-boundary detection.
- Translation is opt-in and clearly separated from transcription.
- Production enrichment uses only pinned models and resources that run on standard free public GitHub runners and free public storage.
- Expensive local model experimentation is optional later work and cannot block canonical archival or trigger a paid hosted fallback.
