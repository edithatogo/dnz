"""Build a public, rights-filtered inventory from DigitalNZ facet aggregates.

This script deliberately produces only rights statements and aggregate record
counts.  The source table does not map individual DigitalNZ objects to rights
statements, so it cannot establish a licence for any underlying object.
"""

from __future__ import annotations

import csv
import json
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "digitalnz" / "facets" / "rights.csv"
OUTPUT = ROOT / "registry" / "dataset" / "public-rights-archive"

APPROVED = {
    "public_domain_or_no_known_restrictions",
    "cc_by_attribution_only",
    "other_open_or_conditional",
    "cc_by_sa",
    "cc_by_nc",
    "cc_by_nc_sa",
    "cc_by_nc_nd",
}


def classify(statement: str) -> str:
    value = statement.casefold()
    compact = "".join(character for character in value if character.isalnum())
    if any(token in compact for token in ("cc0", "publicdomain", "noknowncopyrightrestrictions", "noknownrestrictionsonuse", "noknowncopyright")):
        return "public_domain_or_no_known_restrictions"
    if "ccbyncnd" in compact or "noncommercialnoderiv" in compact:
        return "cc_by_nc_nd"
    if "ccbyncsa" in compact or "noncommercialsharealike" in compact:
        return "cc_by_nc_sa"
    if "ccbync" in compact or "attributionnoncommercial" in compact or "noncommercial" in compact:
        return "cc_by_nc"
    if "ccbysa" in compact or "sharealike" in compact:
        return "cc_by_sa"
    if "ccby" in compact or "creativecommonsattribution" in compact:
        return "cc_by_attribution_only"
    if any(token in value for token in ("permission", "no public use", "no copies", "private study", "written consent")):
        return "permission_required_or_explicit_restriction"
    if "all rights reserved" in value or "protected by copyright" in value:
        return "copyright_reserved_or_all_rights_reserved"
    if "copyright" in value:
        return "copyright_reserved_or_all_rights_reserved"
    if any(token in value for token in ("open", "licence", "license", "reuse", "re-use")):
        return "other_open_or_conditional"
    return "unresolved_some_rights_reserved_or_other"


def main() -> None:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    groups: dict[str, list[dict[str, object]]] = {}
    totals: Counter[str] = Counter()
    with SOURCE.open(encoding="utf-8-sig", newline="") as source:
        for row in csv.DictReader(source):
            statement = row["value"].strip()
            count = int(row["count"])
            category = classify(statement)
            groups.setdefault(category, []).append(
                {"rights_statement": statement, "record_count": count, "rights_category": category}
            )
            totals[category] += count

    for category, rows in groups.items():
        destination = OUTPUT / f"{category}.csv"
        with destination.open("w", encoding="utf-8", newline="") as output:
            writer = csv.DictWriter(output, fieldnames=["rights_statement", "record_count", "rights_category"])
            writer.writeheader()
            writer.writerows(sorted(rows, key=lambda row: (-int(row["record_count"]), str(row["rights_statement"]))))

    manifest = {
        "schema_version": 1,
        "source": "digitalnz/facets/rights.csv",
        "scope": "aggregate rights statements and record counts only; no underlying DigitalNZ objects or provider payloads",
        "approved_public_categories": sorted(APPROVED),
        "review_categories": sorted(set(groups) - APPROVED),
        "record_counts": dict(sorted(totals.items())),
        "total_records": sum(totals.values()),
    }
    (OUTPUT / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print(f"public rights archive: PASS ({len(groups)} categories, {manifest['total_records']} aggregate records)")


if __name__ == "__main__":
    main()
