#!/usr/bin/env python3
"""Build a non-duplicative collection-level archive recommendation map.

The DigitalNZ usage facet is an aggregate cross-tab.  It is valuable
preservation metadata, but it is not a per-item licence ledger: collections can
appear in more than one usage bucket.  This tool therefore produces a map that
is deliberately conservative about copying payloads.
"""

from __future__ import annotations

import csv
from collections import defaultdict
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "digitalnz" / "facets" / "usage_by_collection_and_partner.csv"
OUTPUT = ROOT / "registry" / "dataset" / "digitalnz-collection-archive-map.csv"

# These overrides reflect independently checked source terms, not the aggregate
# DigitalNZ usage facet.  Keep the source-specific decision explicit.
OVERRIDES = {
    "New Zealand Gazette": {
        "tier": "metadata_and_notice_text",
        "recommendation": "Archive metadata and published notice text with DIA attribution; exclude logos, emblems, trademarks, site design, and any material removed or redacted for privacy.",
        "evidence": "Gazette copyright and submission terms license Gazette material for reuse under CC BY 3.0 NZ.",
        "existing_archives": "HF edithatogo/digitalnz (facet metadata only); HF edithatogo/courts-nz-public-notices-archive (different courts-notices corpus; do not merge).",
    },
    "Cenotaph Database": {
        "tier": "metadata_snapshot_pending_primary_licence",
        "recommendation": "Archive descriptive metadata, identifiers, provenance URLs, and hashes now. Do not bulk-copy biographies, attached documents, or media until Auckland Museum confirms the current licence and any cultural/privacy constraints for this endpoint.",
        "evidence": "Auckland Museum API describes Cenotaph as linked open data, but this review did not locate a current first-party blanket reuse licence.",
        "existing_archives": "HF edithatogo/digitalnz (facet metadata only); no exact Cenotaph corpus found in the checked GitHub/HF estate.",
    },
    "Papers Past": {
        "tier": "metadata_and_item_level_payload",
        "recommendation": "Archive metadata and source URLs. Copy OCR, page images, and PDFs only where the individual item/provider terms are permissive or the item is public domain; do not treat collection-level usage as a blanket licence.",
        "evidence": "National Library states copyright varies by item and source.",
        "existing_archives": "HF edithatogo/digitalnz preserves the DigitalNZ facet snapshot; no Papers Past payload corpus was found in the checked estate.",
    },
    "iNaturalist NZ — Mātaki Taiao": {
        "tier": "metadata_and_item_level_payload",
        "recommendation": "Archive observation metadata with provenance. Copy photos, sounds, and observations only when the individual licence allows it; preserve licence, attribution, and observer/source fields.",
        "evidence": "iNaturalist licences observations, photos, and sounds at item level and they may differ.",
        "existing_archives": "HF edithatogo/digitalnz (facet metadata only); no exact iNaturalist NZ corpus found in the checked estate.",
    },
    "Archives New Zealand Historical Film Footage": {
        "tier": "metadata_and_item_level_payload",
        "recommendation": "Archive catalogue metadata, identifiers, and landing URLs. Copy digitised objects only after checking the item rights statement and any access restriction. Apply the same rule to the provider's other collections in the generated map.",
        "evidence": "Aggregate usage signals allow reuse for some records only; they do not establish collection-wide object rights.",
        "existing_archives": "HF edithatogo/digitalnz (facet metadata only); no exact Archway corpus found in the checked estate.",
    },
}


def base_recommendation(usages: set[str]) -> tuple[str, str]:
    if "All rights reserved" in usages or "Unknown" in usages:
        return (
            "metadata_only_pending_item_review",
            "Archive descriptive metadata, identifiers, rights text, landing URLs, timestamps, and hashes. Do not publicly copy payloads until an item-level provider licence is verified.",
        )
    if {"Share", "Modify", "Use commercially"}.issubset(usages):
        return (
            "metadata_plus_open_item_candidates",
            "Archive metadata now. The aggregate facet has open-use signals, but copy payloads only after recording the item-level rights statement and provenance; collection-level totals are not a blanket licence.",
        )
    return (
        "metadata_only_pending_item_review",
        "Archive descriptive metadata, identifiers, rights text, landing URLs, timestamps, and hashes. Aggregate usage is insufficient authority to copy payloads.",
    )


def main() -> None:
    collections: dict[tuple[str, str], dict[str, object]] = defaultdict(
        lambda: {"usages": set(), "items_total": 0, "usage_total": 0}
    )
    with SOURCE.open(encoding="utf-8", newline="") as handle:
        for row in csv.DictReader(handle):
            key = (row["content_partner"], row["primary_collection"])
            record = collections[key]
            record["usages"].add(row["usage"])
            record["items_total"] = max(record["items_total"], int(row["items_total"]))
            record["usage_total"] += int(row["count"])

    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    with OUTPUT.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(
            handle,
            fieldnames=[
                "content_partner", "primary_collection", "items_total",
                "digitalnz_usage_buckets", "archive_tier", "recommendation",
                "rights_evidence", "existing_archive_coverage", "duplication_decision",
            ],
        )
        writer.writeheader()
        for (partner, collection), record in sorted(collections.items(), key=lambda item: item[0][1].casefold()):
            usages = record["usages"]
            tier, recommendation = base_recommendation(usages)
            override = OVERRIDES.get(collection)
            if override:
                tier = override["tier"]
                recommendation = override["recommendation"]
                evidence = override["evidence"]
                coverage = override["existing_archives"]
            else:
                evidence = "DigitalNZ aggregate usage facet only; verify current provider terms and each item rights statement."
                coverage = "HF edithatogo/digitalnz already preserves this DigitalNZ facet snapshot; add only derived decisions and source-specific captures here."
            writer.writerow({
                "content_partner": partner,
                "primary_collection": collection,
                "items_total": record["items_total"],
                "digitalnz_usage_buckets": " | ".join(sorted(usages)),
                "archive_tier": tier,
                "recommendation": recommendation,
                "rights_evidence": evidence,
                "existing_archive_coverage": coverage,
                "duplication_decision": "Do not copy the facet source again; create a new payload archive only when this row permits it and source capture is distinct.",
            })


if __name__ == "__main__":
    main()
