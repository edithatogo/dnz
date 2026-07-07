#!/usr/bin/env python3
"""Generate API documentation and collection inventory pages for the docs site."""

from __future__ import annotations

import argparse
import csv
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class DocEntry:
    source: str
    role: str
    destination: str
    notes: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", help="Path to the repository root.")
    parser.add_argument(
        "--docs-map",
        default="docs/src/content/docs/generated/api-documentation-map.md",
        help="Path to write the API documentation map page.",
    )
    parser.add_argument(
        "--collections-page",
        default="docs/src/content/docs/generated/digitalnz-major-collections.md",
        help="Path to write the major collections page.",
    )
    parser.add_argument(
        "--usage-source",
        default="digitalnz/facets/usage_by_collection_and_partner.csv",
        help="Path to the usage-by-collection facet export.",
    )
    return parser.parse_args()


def read_collection_rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    rows.sort(key=lambda row: int(row["count"]), reverse=True)
    return rows


def read_usage_rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    allowed_usages = {"Use commercially", "Share", "Modify"}
    rows = [row for row in rows if row["usage"] in allowed_usages]
    rows.sort(key=lambda row: (int(row["count"]), int(row["items_total"])), reverse=True)
    return rows


def render_docs_map(entries: list[DocEntry]) -> str:
    lines = [
        "---",
        "title: API Documentation Map",
        "description: Canonical map of the repo's API documentation sources and published docs destinations.",
        "---",
        "",
        "# API Documentation Map",
        "",
        "This page maps the repository's API-related documentation sources into the published Astro/Starlight docs surface and preserves the source artifacts that remain notebook- or data-led.",
        "",
        "## Mapping",
        "",
        "| Source | Role | Docs destination | Notes |",
        "|---|---|---|---|",
    ]
    for entry in entries:
        lines.append(
            f"| `{entry.source}` | {entry.role} | {entry.destination} | {entry.notes} |"
        )

    lines.extend(
        [
            "",
            "## Reading order",
            "",
            "- Start with the docs index and quickstart for end-user guidance.",
            "- Use the architecture and contract pages for boundary and integration detail.",
            "- Treat notebooks and CSVs under `digitalnz/` as historical source artifacts that back the published inventory pages.",
        ]
    )
    return "\n".join(lines) + "\n"


def render_collections_page(rows: list[dict[str, str]], limit: int = 25) -> str:
    selected = rows[:limit]
    lines = [
        "---",
        "title: Major DigitalNZ Collections",
        "description: Rights-safe major collection groups behind the DigitalNZ API, derived from checked-in facet exports.",
        "---",
        "",
        "# Major DigitalNZ Collections",
        "",
        "This inventory includes only usage rows whose checked-in facet is `Use commercially`, `Share`, or `Modify`.",
        "",
        "It is derived from `digitalnz/facets/usage_by_collection_and_partner.csv` and corroborated by the collection visualisation artifacts under `digitalnz/`.",
        "",
        f"The table below lists the top {limit} permitted usage rows by rights-safe item count in the checked-in facet export.",
        "",
        "## Inventory",
        "",
        "| Content partner | Primary collection | Usage | Rights-safe count | Items total | Usage total | Source artifact |",
        "|---|---|---|---:|---:|---:|---|",
    ]
    for row in selected:
        lines.append(
            f"| {row['content_partner']} | {row['primary_collection']} | {row['usage']} | {int(row['count']):,} | {int(row['items_total']):,} | {int(row['usage_total']):,} | `digitalnz/facets/usage_by_collection_and_partner.csv` |"
        )

    lines.extend(
        [
            "",
            "## Notes",
            "",
            "- Counts are snapshot counts from the checked-in facet export, not live API calls.",
            "- The same content partner and collection may appear more than once when it has multiple permitted usage classes.",
            "- `digitalnz/open_collections_digitalnz.html`, `digitalnz/facets/collections_by_partner.csv`, and `digitalnz/facets/rights.csv` provide supporting visualisation and rights context.",
        ]
    )
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    docs_map_entries = [
        DocEntry(
            source="README.md",
            role="Canonical repo overview and entrypoint",
            destination="[docs index](../index.md) and [getting started](./guides/getting-started.md)",
            notes="Use this as the top-level orientation page.",
        ),
        DocEntry(
            source="docs/src/content/docs/guides/getting-started.md",
            role="Canonical quickstart",
            destination="Published as-is",
            notes="Covers API keys, CLI, and MCP startup.",
        ),
        DocEntry(
            source="docs/src/content/docs/guides/architecture.md",
            role="Canonical architecture reference",
            destination="Published as-is",
            notes="Explains crate boundaries and the DigitalNZ API flow.",
        ),
        DocEntry(
            source="docs/src/content/docs/guides/digitalnz-open-social-data-contract.md",
            role="Canonical upstream/downstream contract",
            destination="Published as-is",
            notes="Documents the `dnz` boundary for downstream reuse.",
        ),
        DocEntry(
            source="docs/src/content/docs/guides/registry-submission.md",
            role="Canonical registry workflow guide",
            destination="Published as-is",
            notes="Summarizes release and registry publication workflow.",
        ),
        DocEntry(
            source="pydnz/README.md",
            role="Legacy Python client documentation",
            destination="Retained source artifact",
            notes="Referenced for historical Python client details.",
        ),
        DocEntry(
            source="digitalnz/README.md",
            role="Notebook corpus index",
            destination="Retained source artifact",
            notes="Source hub for the notebook-led exploration material.",
        ),
        DocEntry(
            source="digitalnz/*.ipynb",
            role="Historical notebooks and exploratory examples",
            destination="Retained source artifacts",
            notes="These back the collection and facet inventories.",
        ),
        DocEntry(
            source="digitalnz/facets/*.csv",
            role="Generated collection and facet inventories",
            destination="Retained source artifacts",
            notes="Used to derive the published major collections page.",
        ),
        DocEntry(
            source="digitalnz/open_collections_digitalnz.html",
            role="Visual collection inventory artifact",
            destination="Retained source artifact",
            notes="Provides a visual companion to the collection counts.",
        ),
        DocEntry(
            source="crates/dnz-core/src/*",
            role="Core library implementation docs",
            destination="Explained through architecture and code comments",
            notes="Surface the client, models, export, vector, and digest layers.",
        ),
        DocEntry(
            source="crates/dnz-cli/src/*",
            role="CLI implementation docs",
            destination="Explained through the quickstart and CLI help",
            notes="Maps the user-facing CLI surface to published docs.",
        ),
        DocEntry(
            source="crates/dnz-mcp/src/*",
            role="MCP server implementation docs",
            destination="Explained through the quickstart and registry docs",
            notes="Maps the stdio server surface to published docs.",
        ),
        DocEntry(
            source="crates/dnz-python/src/*",
            role="Python bindings implementation docs",
            destination="Explained through the README and architecture docs",
            notes="Maps the FFI surface to published docs.",
        ),
    ]

    docs_map_path = repo_root / args.docs_map
    collections_path = repo_root / args.collections_page
    collections_source = repo_root / "digitalnz" / "facets" / "usage_by_collection_and_partner.csv"
    collections_rows = read_usage_rows(collections_source)

    docs_map_path.parent.mkdir(parents=True, exist_ok=True)
    collections_path.parent.mkdir(parents=True, exist_ok=True)
    docs_map_path.write_text(render_docs_map(docs_map_entries), encoding="utf-8")
    collections_path.write_text(render_collections_page(collections_rows), encoding="utf-8")

    print(f"Wrote {docs_map_path}")
    print(f"Wrote {collections_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
