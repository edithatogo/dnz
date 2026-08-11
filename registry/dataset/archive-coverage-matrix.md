# DigitalNZ archive coverage and capture decisions

This is the single operational map for avoiding duplicate preservation work.
The exhaustive, generated collection-level decisions are in
[`digitalnz-collection-archive-map.csv`](digitalnz-collection-archive-map.csv).
They derive from the DigitalNZ `usage_by_collection_and_partner` facet captured
in this repository on the recorded source date.

## Existing archives

| Archive | What it already preserves | Decision |
| --- | --- | --- |
| [HF `edithatogo/digitalnz`](https://huggingface.co/datasets/edithatogo/digitalnz) | Full DigitalNZ facet snapshot, including rights, collection, and usage facets, plus this repository's registry metadata. | Canonical copy for the facet source. Do not upload a second copy of those CSVs. Add only derived decisions and genuinely distinct, rights-cleared source captures. |
| [GitHub `edithatogo/digitalnz`](https://github.com/edithatogo/digitalnz) | GLAM Workbench notebooks and the upstream facet-oriented DigitalNZ working copy. | Tooling/source companion; do not treat it as a separate payload archive. |
| [HF `edithatogo/nz-legislation-corpus`](https://huggingface.co/datasets/edithatogo/nz-legislation-corpus) and [HF `edithatogo/corpus-legislation-nz`](https://huggingface.co/datasets/edithatogo/corpus-legislation-nz) | Official NZ Legislation API corpus and its frozen DOI snapshot. | Retain as the canonical legislation archive. Gazette notices are a different publication stream, so only Gazette-specific notices should be captured separately. |
| [GitHub `edithatogo/archive-govt-nz`](https://github.com/edithatogo/archive-govt-nz) and [HF `edithatogo/archive-govt-nz-treasury`](https://huggingface.co/datasets/edithatogo/archive-govt-nz-treasury) | CKAN/Treasury archival tooling and a Treasury release. | No overlap with DigitalNZ collection payloads. Reuse its evidence/manifest model if a new government-source capture is made. |
| [HF `edithatogo/courts-nz-public-notices-archive`](https://huggingface.co/datasets/edithatogo/courts-nz-public-notices-archive) | Courts public-notices corpus. | Keep distinct from Gazette: overlap in notice-like material is not identity or licence equivalence. |

## Updated priority recommendations

| Collection | DigitalNZ signal | Capture recommendation | Why |
| --- | --- | --- | --- |
| New Zealand Gazette | Share, Modify, Use commercially | Archive metadata and published notice text, with a manifest, source URL, retrieval time, content hash, and DIA/CC BY 3.0 NZ attribution. Exclude logos, emblems, trade marks, site design, and honour removals/redactions. | Gazette's copyright and submitter terms expressly license published Gazette material for reuse under CC BY 3.0 NZ. |
| Cenotaph Database | Share, Modify, Use commercially | Preserve metadata, stable identifiers, source URLs, and hashes now. Hold bulk biographies, documents, and media until Auckland Museum confirms the current blanket licence and endpoint-specific constraints. | The API describes linked open data, but this review did not find a current first-party blanket reuse statement. Third-party references to CC BY are useful leads, not a sufficient archival authority. |
| Papers Past | Mixed, including All rights reserved | Preserve metadata and provenance. Capture OCR/images/PDFs only for public-domain or expressly permissive individual items. | National Library guidance says rights vary by material/source; the collection-level usage facet overlaps and cannot license all content. |
| iNaturalist NZ — Mātaki Taiao | Mixed, including All rights reserved | Preserve observation metadata and licences. Copy observations/media only where each individual licence permits it, retaining attribution and source fields. | Observation, photograph, and sound licences can differ by item. |
| Auckland Museum Collections, Te Papa Collections Online, Auckland Libraries Heritage Images Collection, NZ Electronic Text Collection, Archives New Zealand collections | Mixed or provider-specific | Metadata-first; queue an item-level rights resolver before payload capture. | These are cultural-heritage aggregations with record-level variation and possible cultural, privacy, or third-party constraints. |
| data.govt.nz, Figure NZ | Mostly open-use signals | Preserve metadata now; start a source-specific terms check, then create distinct archives only for datasets not already in the estate. | They are portals/derivative publishers, not a licence proof for every underlying resource. |

## Archive tiers

- `metadata_only_pending_item_review`: store descriptive metadata, identifiers,
  rights text, provider URL, retrieval timestamp, and hashes; no payload.
- `metadata_plus_open_item_candidates`: record-level open-use signals exist, but
  capture each payload only with its contemporary rights statement.
- `metadata_and_item_level_payload`: provider guidance says rights vary; payload
  is allowed only where the specific item is permissive.
- `metadata_and_notice_text`: source terms permit the published text; still
  enforce stated exclusions and privacy/removal handling.
- `metadata_snapshot_pending_primary_licence`: preserve a verifiable catalogue
  snapshot while seeking a first-party licence before bulk payload copying.

The usage facet has overlapping buckets, so no row in the generated map is a
blanket licence for a whole collection. The public rights inventory remains
aggregate metadata rather than a list of objects cleared for redistribution.
