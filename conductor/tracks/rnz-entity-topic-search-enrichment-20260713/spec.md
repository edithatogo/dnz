# Track Specification: RNZ Entity, Topic & Search Enrichment

## Requirements

- Extract people, organisations, places, legislation, ministries, parties, programmes, iwi and hapū from transcript text and supplied metadata.
- Link only high-confidence entities to authoritative identifiers and retain unresolved candidates.
- Produce hierarchical public-policy topics, timestamped full-text indexes and transcript-cited summaries.
- Support exact, fuzzy and phonetic retrieval without performing voice identification.
- Version every model, vocabulary and authority dataset used by enrichment.

## Acceptance Criteria

- Every entity, topic and summary claim links to transcript spans and confidence.
- Search indexes preserve canonical word and recording offsets.
- Evaluation reports precision and recall on a manually reviewed sample.
