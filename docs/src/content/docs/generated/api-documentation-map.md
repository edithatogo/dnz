---
title: API Documentation Map
description: Canonical map of the repo's API documentation sources and published docs destinations.
---

# API Documentation Map

This page maps the repository's API-related documentation sources into the published Astro/Starlight docs surface and preserves the source artifacts that remain notebook- or data-led.

## Mapping

| Source | Role | Docs destination | Notes |
|---|---|---|---|
| `README.md` | Canonical repo overview and entrypoint | [docs index](../index.md) and [getting started](./guides/getting-started.md) | Use this as the top-level orientation page. |
| `docs/src/content/docs/guides/getting-started.md` | Canonical quickstart | Published as-is | Covers API keys, CLI, and MCP startup. |
| `docs/src/content/docs/guides/architecture.md` | Canonical architecture reference | Published as-is | Explains crate boundaries and the DigitalNZ API flow. |
| `docs/src/content/docs/guides/digitalnz-open-social-data-contract.md` | Canonical upstream/downstream contract | Published as-is | Documents the `dnz` boundary for downstream reuse. |
| `docs/src/content/docs/guides/registry-submission.md` | Canonical registry workflow guide | Published as-is | Summarizes release and registry publication workflow. |
| `pydnz/README.md` | Legacy Python client documentation | Retained source artifact | Referenced for historical Python client details. |
| `digitalnz/README.md` | Notebook corpus index | Retained source artifact | Source hub for the notebook-led exploration material. |
| `digitalnz/*.ipynb` | Historical notebooks and exploratory examples | Retained source artifacts | These back the collection and facet inventories. |
| `digitalnz/facets/*.csv` | Generated collection and facet inventories | Retained source artifacts | Used to derive the published major collections page. |
| `digitalnz/open_collections_digitalnz.html` | Visual collection inventory artifact | Retained source artifact | Provides a visual companion to the collection counts. |
| `digitalnz/facets/usage_by_collection_and_partner.csv` | Collection-family rights metadata | Retained source artifact | Used to document the RNZ family as metadata-only coverage. |
| `docs/src/content/docs/generated/rnz-recordings-and-docs.md` | RNZ family documentation page | Published as-is | Records the Radio New Zealand collection rows without asserting media redistribution rights. |
| `crates/dnz-core/src/*` | Core library implementation docs | Explained through architecture and code comments | Surface the client, models, export, vector, and digest layers. |
| `crates/dnz-cli/src/*` | CLI implementation docs | Explained through the quickstart and CLI help | Maps the user-facing CLI surface to published docs. |
| `crates/dnz-mcp/src/*` | MCP server implementation docs | Explained through the quickstart and registry docs | Maps the stdio server surface to published docs. |
| `crates/dnz-python/src/*` | Python bindings implementation docs | Explained through the README and architecture docs | Maps the FFI surface to published docs. |

## Reading order

- Start with the docs index and quickstart for end-user guidance.
- Use the architecture and contract pages for boundary and integration detail.
- Treat notebooks and CSVs under `digitalnz/` and the legacy source tree under `pydnz/` as historical source artifacts that back the published inventory pages and source archive snapshots.
- Use the RNZ family page as metadata-only documentation for the checked-in Radio New Zealand rows.
